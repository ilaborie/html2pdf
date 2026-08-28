//! End-to-end rendering tests.
//!
//! These launch a real headless browser, so they are `#[ignore]`d: CI has no guaranteed Chrome
//! installation. Run them with `just test-e2e`.
//!
//! If browser auto-detection picks the wrong binary, point `CHROME` at a working executable.

use std::path::{Path, PathBuf};
use std::time::Duration;

use assert2::check;
use html2pdf::{html_to_pdf, BrowserOptions, PaperSize, PdfOptions, WaitFor};
use rstest::rstest;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// A PDF always starts with the `%PDF-` magic bytes.
fn check_is_pdf(output: &Path) {
    let bytes = std::fs::read(output).expect("should read the generated PDF");
    check!(bytes.len() > 1_000, "the PDF should not be a stub");
    check!(bytes.starts_with(b"%PDF-"), "should be a PDF");
}

#[rstest]
#[case::navigation(WaitFor::Navigation)]
#[case::load(WaitFor::Load)]
#[case::network_idle(WaitFor::NetworkIdle)]
#[ignore = "launches a real browser"]
#[tokio::test]
async fn should_render_a_pdf_for_each_wait_for(#[case] wait_for: WaitFor) {
    let dir = tempfile::tempdir().expect("should create a temp dir");
    let output = dir.path().join("out.pdf");

    html_to_pdf(
        fixture("example.html"),
        &output,
        PdfOptions {
            print_background: true,
            paper: Some(PaperSize::A4),
            header_template: Some(String::from("<span class=title></span>")),
            ..PdfOptions::default()
        },
        BrowserOptions {
            wait_for,
            ..BrowserOptions::default()
        },
    )
    .await
    .expect("should render the PDF");

    check_is_pdf(&output);
}

/// The fixed `--wait` delay still applies on top of the lifecycle milestone.
#[ignore = "launches a real browser"]
#[tokio::test]
async fn should_apply_the_extra_wait() {
    let dir = tempfile::tempdir().expect("should create a temp dir");
    let output = dir.path().join("out.pdf");

    let start = std::time::Instant::now();
    html_to_pdf(
        fixture("example.html"),
        &output,
        PdfOptions::default(),
        BrowserOptions {
            wait: Some(Duration::from_secs(2)),
            ..BrowserOptions::default()
        },
    )
    .await
    .expect("should render the PDF");

    check!(start.elapsed() >= Duration::from_secs(2));
    check_is_pdf(&output);
}

/// A page whose network never goes quiet must still produce a PDF: the lifecycle wait falls back
/// to printing rather than hanging forever. Takes ~20s, the length of the internal ceiling.
#[ignore = "launches a real browser, and deliberately waits out the lifecycle timeout"]
#[tokio::test]
async fn should_print_anyway_when_the_page_never_goes_idle() {
    let dir = tempfile::tempdir().expect("should create a temp dir");
    let output = dir.path().join("out.pdf");

    html_to_pdf(
        fixture("never-idle.html"),
        &output,
        PdfOptions::default(),
        BrowserOptions {
            wait_for: WaitFor::NetworkIdle,
            ..BrowserOptions::default()
        },
    )
    .await
    .expect("should render the PDF despite the page never settling");

    check_is_pdf(&output);
}
