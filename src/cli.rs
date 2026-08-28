use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use clap::Parser;
use humantime::parse_duration;

use crate::Error;

/// Generate a PDF from a local HTML file using a headless chrome
#[derive(Debug, Parser)]
#[clap(version)]
pub struct Options {
    /// Input HTML file.
    pub input: PathBuf,

    /// Output file.
    /// By default, just change the input extension to PDF
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Use landscape mode.
    #[clap(long)]
    pub landscape: bool,

    /// Allow print background.
    #[clap(long)]
    pub background: bool,

    /// Time to wait in ms before printing.
    /// Examples: 150ms, 10s
    #[clap(long, value_parser = parse_duration)]
    pub wait: Option<Duration>,

    /// When to consider the page ready for printing.
    /// Supported values: navigation, load, network-idle
    #[clap(long, default_value = "navigation")]
    pub wait_for: WaitFor,

    /// HTML template for the print header.
    /// Should be valid HTML markup with following classes used to inject printing values into
    /// them:
    /// date for formatted print date,
    /// title for document title,
    /// url for document location,
    /// pageNumber for current page number,
    /// totalPages for total pages in the document.
    /// For example, `<span class=title></span>` would generate span containing the title.
    #[clap(long)]
    pub header: Option<String>,

    /// HTML template for the print footer.
    /// Should use the same format as the headerTemplate.
    #[clap(long)]
    pub footer: Option<String>,

    /// Paper size.
    /// Supported values: A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6
    #[clap(long)]
    pub paper: Option<PaperSize>,

    /// Scale, default to 1.0
    #[clap(long)]
    pub scale: Option<f64>,

    /// Paper ranges to print,
    /// e.g. '1-5, 8, 11-13'
    #[clap(long)]
    pub range: Option<String>,

    /// Margin in inches
    /// You can define margin like this:
    /// '0.4' the value is applied for all side,
    /// '0.4 0.4' : first value is applied for top and bottom, second for left and right,
    /// '0.4 0.4 0.4 0.4' : first value is applied for top then, right, then bottom, and last for left
    #[clap(long)]
    pub margin: Option<Margin>,

    /// Disable Chrome sandbox
    /// Not recommended, unless running on docker
    #[clap(long)]
    pub disable_sandbox: bool,
}

impl From<&Options> for PdfOptions {
    fn from(opt: &Options) -> Self {
        Self {
            landscape: opt.landscape,
            print_background: opt.background,
            scale: opt.scale,
            paper: opt.paper,
            margin: opt.margin.clone(),
            page_ranges: opt.range.clone(),
            header_template: opt.header.clone(),
            footer_template: opt.footer.clone(),
        }
    }
}

impl From<&Options> for BrowserOptions {
    fn from(opt: &Options) -> Self {
        Self {
            disable_sandbox: opt.disable_sandbox,
            wait_for: opt.wait_for,
            wait: opt.wait,
        }
    }
}

/// PDF printing options, independent of the browser backend.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PdfOptions {
    /// Use landscape mode.
    pub landscape: bool,

    /// Print background graphics.
    pub print_background: bool,

    /// Scale of the page rendering, defaults to 1.0.
    pub scale: Option<f64>,

    /// Paper size. When unset, the browser default (Letter) applies.
    pub paper: Option<PaperSize>,

    /// Margins in inches. When unset, the browser default (~0.4in) applies.
    pub margin: Option<Margin>,

    /// Paper ranges to print, e.g. `1-5, 8, 11-13`.
    pub page_ranges: Option<String>,

    /// HTML template for the print header.
    pub header_template: Option<String>,

    /// HTML template for the print footer.
    pub footer_template: Option<String>,
}

impl PdfOptions {
    /// Whether the header/footer band should be rendered at all.
    ///
    /// Chrome only honours the header and footer templates when this is set, so it is derived
    /// rather than exposed: it is on exactly when at least one template is provided.
    #[must_use]
    pub fn display_header_footer(&self) -> bool {
        self.header_template.is_some() || self.footer_template.is_some()
    }
}

/// How the browser is launched, and when the page is considered ready to print.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BrowserOptions {
    /// Disable the Chrome sandbox.
    /// Not recommended, unless running on docker.
    pub disable_sandbox: bool,

    /// The page lifecycle milestone to wait for before printing.
    pub wait_for: WaitFor,

    /// An extra fixed delay applied *after* `wait_for` has settled.
    pub wait: Option<Duration>,
}

/// When to consider the page ready for printing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, derive_more::Display)]
pub enum WaitFor {
    /// Print as soon as navigation completes.
    #[default]
    #[display("navigation")]
    Navigation,

    /// Wait for the `load` lifecycle event, i.e. sub-resources have been fetched.
    #[display("load")]
    Load,

    /// Wait for the `networkIdle` lifecycle event, i.e. web fonts, images and XHR have settled.
    #[display("network-idle")]
    NetworkIdle,
}

impl FromStr for WaitFor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "navigation" => Ok(Self::Navigation),
            "load" => Ok(Self::Load),
            "network-idle" | "networkidle" => Ok(Self::NetworkIdle),
            _ => Err(Error::InvalidWaitFor {
                value: s.to_string(),
            }),
        }
    }
}
/// Paper size
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub enum PaperSize {
    /// A0 (84.1cm × 118.9cm)
    A0,

    /// A1 (59.4cm × 84.1cm)
    A1,

    /// A2 (42.0cm × 59.4cm)
    A2,

    /// A3 (29.7cm × 42.0cm)
    A3,

    /// A4 (21.0cm × 29.7 cm)
    A4,

    /// A5 (14.8cm × 21.0cm)
    A5,

    /// A6 (10.5cm × 14.8cm)
    A6,

    /// US Letter (11.0in × 8.5in)
    Letter,

    /// Legal (17in × 8.5in)
    Legal,

    /// Tabloid (17in × 11in)
    Tabloid,
}

impl PaperSize {
    /// Returns `(width_inches, height_inches)` for this paper size.
    #[must_use]
    pub fn dimensions(self) -> (f64, f64) {
        match self {
            Self::A0 => (33.1, 46.8),
            Self::A1 => (23.4, 33.1),
            Self::A2 => (16.5, 23.4),
            Self::A3 => (11.7, 16.5),
            Self::A4 => (8.27, 11.7),
            Self::A5 => (5.83, 8.27),
            Self::A6 => (4.13, 5.83),
            Self::Letter => (8.5, 11.0),
            Self::Legal => (8.5, 17.0),
            Self::Tabloid => (11.0, 17.0),
        }
    }

    /// Width in inches.
    #[must_use]
    pub fn paper_width(self) -> f64 {
        self.dimensions().0
    }

    /// Height in inches.
    #[must_use]
    pub fn paper_height(self) -> f64 {
        self.dimensions().1
    }
}

impl FromStr for PaperSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a0" => Ok(Self::A0),
            "a1" => Ok(Self::A1),
            "a2" => Ok(Self::A2),
            "a3" => Ok(Self::A3),
            "a4" => Ok(Self::A4),
            "a5" => Ok(Self::A5),
            "a6" => Ok(Self::A6),
            "letter" => Ok(Self::Letter),
            "legal" => Ok(Self::Legal),
            "tabloid" => Ok(Self::Tabloid),
            _ => Err(Error::InvalidPaperSize {
                size: s.to_string(),
            }),
        }
    }
}

/// Margin definition in inches (top, right, bottom, left)
#[derive(Debug, Clone, PartialEq)]
pub struct Margin {
    /// Top margin
    pub top: f64,
    /// Right margin
    pub right: f64,
    /// Bottom margin
    pub bottom: f64,
    /// Left margin
    pub left: f64,
}

impl FromStr for Margin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(' ').filter(|s| !s.is_empty()).collect();
        match values.len() {
            1 => {
                let all = s.parse::<f64>()?;
                Ok(Self {
                    top: all,
                    right: all,
                    bottom: all,
                    left: all,
                })
            }
            2 => {
                let v = values[0].parse::<f64>()?;
                let h = values[1].parse::<f64>()?;
                Ok(Self {
                    top: v,
                    right: h,
                    bottom: v,
                    left: h,
                })
            }
            4 => Ok(Self {
                top: values[0].parse::<f64>()?,
                right: values[1].parse::<f64>()?,
                bottom: values[2].parse::<f64>()?,
                left: values[3].parse::<f64>()?,
            }),
            _ => Err(Error::InvalidMarginDefinition {
                margin: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::a0("a0", PaperSize::A0)]
    #[case::a1("A1", PaperSize::A1)]
    #[case::a2("A2", PaperSize::A2)]
    #[case::a3("A3", PaperSize::A3)]
    #[case::a4("A4", PaperSize::A4)]
    #[case::a5("A5", PaperSize::A5)]
    #[case::a6("A6", PaperSize::A6)]
    #[case::letter("letter", PaperSize::Letter)]
    #[case::legal("Legal", PaperSize::Legal)]
    #[case::tabloid("Tabloid", PaperSize::Tabloid)]
    fn should_parse_valid_paper_size(#[case] value: &str, #[case] expected: PaperSize) {
        let result = value
            .parse::<PaperSize>()
            .expect("should parse valid paper size");
        check!(result == expected);
    }

    #[test]
    fn should_reject_invalid_paper_size() {
        let value = "plop";
        let result = value.parse::<PaperSize>();
        check!(let Err(Error::InvalidPaperSize { .. }) = result);
    }

    #[test]
    fn should_parse_valid_margin_all() {
        let m = "0.4".parse::<Margin>().expect("should parse margin");
        check!(m.top == 0.4);
        check!(m.right == 0.4);
        check!(m.bottom == 0.4);
        check!(m.left == 0.4);
    }

    #[test]
    fn should_parse_valid_margin_vh() {
        let m = "0.4  0.7".parse::<Margin>().expect("should parse margin");
        check!(m.top == 0.4);
        check!(m.bottom == 0.4);
        check!(m.right == 0.7);
        check!(m.left == 0.7);
    }

    #[test]
    fn should_parse_valid_margin_trbl() {
        let m = "0.2   0.3 0.4  0.5"
            .parse::<Margin>()
            .expect("should parse margin");
        check!(m.top == 0.2);
        check!(m.right == 0.3);
        check!(m.bottom == 0.4);
        check!(m.left == 0.5);
    }

    #[test]
    fn should_reject_invalid_margin() {
        let value = "0.2    0.3  0.4";
        let result = value.parse::<Margin>();
        check!(let Err(Error::InvalidMarginDefinition { .. }) = result);
    }

    #[rstest]
    #[case::navigation("navigation", WaitFor::Navigation)]
    #[case::navigation_mixed_case("Navigation", WaitFor::Navigation)]
    #[case::load("load", WaitFor::Load)]
    #[case::network_idle("network-idle", WaitFor::NetworkIdle)]
    #[case::network_idle_mixed_case("Network-Idle", WaitFor::NetworkIdle)]
    #[case::network_idle_compact("networkidle", WaitFor::NetworkIdle)]
    fn should_parse_valid_wait_for(#[case] value: &str, #[case] expected: WaitFor) {
        let result = value
            .parse::<WaitFor>()
            .expect("should parse valid wait-for");
        check!(result == expected);
    }

    #[test]
    fn should_reject_invalid_wait_for() {
        let value = "plop";
        let result = value.parse::<WaitFor>();
        check!(let Err(Error::InvalidWaitFor { .. }) = result);
    }

    /// `WaitFor` round-trips through its `Display`, so the clap `default_value` string and the
    /// `FromStr` arms cannot drift apart.
    #[rstest]
    #[case::navigation(WaitFor::Navigation)]
    #[case::load(WaitFor::Load)]
    #[case::network_idle(WaitFor::NetworkIdle)]
    fn should_round_trip_wait_for(#[case] wait_for: WaitFor) {
        let rendered = wait_for.to_string();
        let parsed = rendered.parse::<WaitFor>().expect("should re-parse");
        check!(parsed == wait_for);
    }

    fn options(input: &str) -> Options {
        Options::try_parse_from(["html2pdf", input]).expect("should parse minimal args")
    }

    #[test]
    fn should_default_wait_for_to_navigation() {
        let opt = options("a.html");
        check!(opt.wait_for == WaitFor::Navigation);
    }

    #[test]
    fn should_map_options_to_pdf_options() {
        let opt = Options::try_parse_from([
            "html2pdf",
            "a.html",
            "--landscape",
            "--background",
            "--paper",
            "A4",
            "--scale",
            "1.5",
            "--range",
            "1-2",
            "--margin",
            "0.4",
            "--header",
            "<span class=title></span>",
        ])
        .expect("should parse args");

        let pdf = PdfOptions::from(&opt);
        check!(pdf.landscape == true);
        check!(pdf.print_background == true);
        check!(pdf.scale == Some(1.5));
        check!(pdf.paper == Some(PaperSize::A4));
        check!(pdf.page_ranges == Some(String::from("1-2")));
        check!(pdf.header_template == Some(String::from("<span class=title></span>")));
        check!(pdf.footer_template == None);
        check!(pdf.margin.as_ref().map(|m| m.top) == Some(0.4));
    }

    /// Chrome ignores the header/footer templates unless `displayHeaderFooter` is set, so this is
    /// derived from them rather than being separately settable.
    #[rstest]
    #[case::neither(None, None, false)]
    #[case::header_only(Some("h"), None, true)]
    #[case::footer_only(None, Some("f"), true)]
    #[case::both(Some("h"), Some("f"), true)]
    fn should_display_header_footer_iff_a_template_is_set(
        #[case] header: Option<&str>,
        #[case] footer: Option<&str>,
        #[case] expected: bool,
    ) {
        let pdf = PdfOptions {
            header_template: header.map(String::from),
            footer_template: footer.map(String::from),
            ..PdfOptions::default()
        };
        check!(pdf.display_header_footer() == expected);
    }

    #[test]
    fn should_map_options_to_browser_options() {
        let opt = Options::try_parse_from([
            "html2pdf",
            "a.html",
            "--disable-sandbox",
            "--wait",
            "2s",
            "--wait-for",
            "network-idle",
        ])
        .expect("should parse args");

        let browser = BrowserOptions::from(&opt);
        check!(browser.disable_sandbox == true);
        check!(browser.wait == Some(Duration::from_secs(2)));
        check!(browser.wait_for == WaitFor::NetworkIdle);
    }

    /// The sandbox must stay ON by default: `BrowserOptions::default()` is the safe configuration.
    #[test]
    fn should_keep_sandbox_enabled_by_default() {
        let browser = BrowserOptions::from(&options("a.html"));
        check!(browser.disable_sandbox == false);
        check!(BrowserOptions::default().disable_sandbox == false);
    }

    #[test]
    fn should_reject_invalid_margin_value() {
        let value = "plop";
        let result = value.parse::<Margin>();
        check!(let Err(Error::InvalidMarginValue(_)) = result);
    }
}
