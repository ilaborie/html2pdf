//! The `chromiumoxide` backend.
//!
//! This is deliberately the only module that names `chromiumoxide`: keeping the browser crate
//! behind this boundary is what lets [`crate::PdfOptions`] and [`crate::BrowserOptions`] stay
//! backend-neutral, so swapping the backend again is not a breaking change.

use std::time::Duration;

use chromiumoxide::cdp::browser_protocol::page::{EventLifecycleEvent, PrintToPdfParams};
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use tempfile::{Builder, TempDir};
use tracing::{debug, info, warn};

use crate::{BrowserOptions, Error, PaperSize, PdfOptions, WaitFor};

/// Ceiling on how long we wait for a lifecycle milestone.
///
/// A page that long-polls, or that has a request which never completes, never reaches
/// `networkIdle`. Matching `chromiumoxide`'s own launch timeout keeps the two in the same scale.
const WAIT_FOR_TIMEOUT: Duration = Duration::from_secs(20);

impl From<&PdfOptions> for PrintToPdfParams {
    fn from(opt: &PdfOptions) -> Self {
        Self {
            landscape: Some(opt.landscape),
            display_header_footer: Some(opt.display_header_footer()),
            print_background: Some(opt.print_background),
            scale: opt.scale,
            paper_width: opt.paper.map(PaperSize::paper_width),
            paper_height: opt.paper.map(PaperSize::paper_height),
            margin_top: opt.margin.as_ref().map(|m| m.top),
            margin_bottom: opt.margin.as_ref().map(|m| m.bottom),
            margin_left: opt.margin.as_ref().map(|m| m.left),
            margin_right: opt.margin.as_ref().map(|m| m.right),
            page_ranges: opt.page_ranges.clone(),
            header_template: opt.header_template.clone(),
            footer_template: opt.footer_template.clone(),
            ..Default::default()
        }
    }
}

/// Build the browser config, along with the throw-away profile directory it uses.
///
/// `chromiumoxide` otherwise defaults every launch to a single shared `chromiumoxide-runner`
/// directory, and Chrome refuses to start a second instance against a profile already held by a
/// `SingletonLock`. Giving each launch its own directory keeps concurrent conversions working.
/// The returned [`TempDir`] must outlive the browser: dropping it deletes the profile.
fn browser_config(opt: &BrowserOptions) -> Result<(BrowserConfig, TempDir), Error> {
    let user_data_dir = Builder::new().prefix("html2pdf-").tempdir()?;

    let mut builder = BrowserConfig::builder().user_data_dir(user_data_dir.path());
    if opt.disable_sandbox {
        builder = builder.no_sandbox();
    }
    let config = builder.build().map_err(Error::browser)?;

    Ok((config, user_data_dir))
}

/// Navigate to `file_url`, wait for the page to settle, and print it to PDF bytes.
pub(crate) async fn print_to_pdf(
    file_url: &str,
    pdf_options: &PdfOptions,
    browser_options: &BrowserOptions,
) -> Result<Vec<u8>, Error> {
    // Held until after `browser.close()`: dropping it removes the profile directory.
    let (config, _user_data_dir) = browser_config(browser_options)?;
    let (mut browser, mut handler) = Browser::launch(config).await.map_err(Error::browser)?;

    // The handler drives the websocket; without it nothing below ever resolves.
    let handle = tokio::spawn(async move {
        while let Some(event) = handler.next().await {
            if event.is_err() {
                break;
            }
        }
    });

    let bytes = render(&browser, file_url, pdf_options, browser_options).await;

    // Closing explicitly: `Drop` only warns and leaves the process to be reaped in the background.
    browser.close().await.map_err(Error::browser)?;
    handle.await.map_err(Error::browser)?;

    bytes
}

async fn render(
    browser: &Browser,
    file_url: &str,
    pdf_options: &PdfOptions,
    browser_options: &BrowserOptions,
) -> Result<Vec<u8>, Error> {
    // Open a blank page first so the lifecycle listener is subscribed *before* navigation starts,
    // otherwise the milestone can fire before the stream exists and the wait below never resolves.
    let page = browser
        .new_page("about:blank")
        .await
        .map_err(Error::browser)?;
    let lifecycle = match browser_options.wait_for {
        WaitFor::Navigation => None,
        WaitFor::Load | WaitFor::NetworkIdle => Some(
            page.event_listener::<EventLifecycleEvent>()
                .await
                .map_err(Error::browser)?,
        ),
    };

    page.goto(file_url).await.map_err(Error::browser)?;
    page.wait_for_navigation().await.map_err(Error::browser)?;

    if let Some(events) = lifecycle {
        wait_for_lifecycle(events, browser_options.wait_for).await;
    }

    if let Some(wait) = browser_options.wait {
        info!(?wait, "Waiting before export to PDF");
        tokio::time::sleep(wait).await;
    }

    let params = PrintToPdfParams::from(pdf_options);
    debug!(?params, "Using PDF options");
    page.pdf(params).await.map_err(Error::browser)
}

/// Consume lifecycle events until `wait_for` is reached, or the timeout expires.
///
/// A timeout is not fatal: printing a slightly early page beats returning nothing at all, so this
/// warns and lets the caller carry on.
async fn wait_for_lifecycle<S>(mut events: S, wait_for: WaitFor)
where
    S: futures::Stream<Item = std::sync::Arc<EventLifecycleEvent>> + Unpin,
{
    // CDP lifecycle event names, see https://chromedevtools.github.io/devtools-protocol/tot/Page/
    let expected = match wait_for {
        WaitFor::Load => "load",
        WaitFor::NetworkIdle | WaitFor::Navigation => "networkIdle",
    };

    let settled = tokio::time::timeout(WAIT_FOR_TIMEOUT, async {
        while let Some(event) = events.next().await {
            debug!(name = %event.name, "Lifecycle event");
            if event.name == expected {
                return;
            }
        }
    })
    .await;

    if settled.is_ok() {
        info!(%wait_for, "Page settled");
    } else {
        warn!(
            %wait_for,
            timeout = ?WAIT_FOR_TIMEOUT,
            "Page never settled, printing anyway"
        );
    }
}
