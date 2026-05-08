use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use clap::Parser;
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::LaunchOptions;
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

impl From<&Options> for PrintToPdfOptions {
    fn from(opt: &Options) -> Self {
        PrintToPdfOptions {
            landscape: Some(opt.landscape),
            display_header_footer: Some(opt.header.is_some() || opt.footer.is_some()),
            print_background: Some(opt.background),
            scale: opt.scale,
            paper_width: opt.paper.map(|p| p.dimensions().0),
            paper_height: opt.paper.map(|p| p.dimensions().1),
            margin_top: opt.margin.as_ref().map(|m| m.top),
            margin_bottom: opt.margin.as_ref().map(|m| m.bottom),
            margin_left: opt.margin.as_ref().map(|m| m.left),
            margin_right: opt.margin.as_ref().map(|m| m.right),
            page_ranges: opt.range.clone(),
            header_template: opt.header.clone(),
            footer_template: opt.footer.clone(),
            ..Default::default()
        }
    }
}

impl From<&Options> for LaunchOptions<'_> {
    fn from(opt: &Options) -> Self {
        LaunchOptions {
            sandbox: !opt.disable_sandbox,
            idle_browser_timeout: opt.wait.unwrap_or_default().max(Duration::from_secs(30)),
            ..Default::default()
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

    #[test]
    fn should_reject_invalid_margin_value() {
        let value = "plop";
        let result = value.parse::<Margin>();
        check!(let Err(Error::InvalidMarginValue(_)) = result);
    }
}
