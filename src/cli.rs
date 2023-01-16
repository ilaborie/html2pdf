use core::f32;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use headless_chrome::protocol::page::PrintToPdfOptions;
use humantime::parse_duration;
use structopt::StructOpt;

use crate::Error;

/// Generate a PDF from a local HTML file using a headless chrome
#[derive(Debug, StructOpt)]
pub struct Options {
    /// Input HTML file.
    #[structopt()]
    pub input: PathBuf,

    /// Output file.
    /// By default, just change the input extension to PDF
    #[structopt(short, long)]
    pub output: Option<PathBuf>,

    /// Use landscape mode.
    #[structopt(long)]
    pub landscape: bool,

    /// Allow print background.
    #[structopt(long)]
    pub background: bool,

    /// Time to wait in ms before printing.
    /// Examples: 150ms, 10s
    #[structopt(long, parse(try_from_str = parse_duration))]
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
    #[structopt(long)]
    pub header: Option<String>,

    /// HTML template for the print footer.
    /// Should use the same format as the headerTemplate.
    #[structopt(long)]
    pub footer: Option<String>,

    /// Paper size.
    /// Supported values: A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6
    #[structopt(long)]
    pub paper: Option<PaperSize>,

    /// Scale, default to 1.0
    #[structopt(long)]
    pub scale: Option<f32>,

    /// Paper ranges to print,
    /// e.g. '1-5, 8, 11-13'
    #[structopt(long)]
    pub range: Option<String>,

    /// Margin in inches
    /// You can define margin like this:
    /// '0.4' the value is applied for all side,
    /// '0.4 0.4' : first value is applied for top and bottom, second for left and right,
    /// '0.4 0.4 0.4 0.4' : first value is applied for top then, right, then bottom, and last for left
    #[structopt(long)]
    pub margin: Option<Margin>,
}

impl Options {
    /// Get a reference to the cli options's input.
    #[must_use]
    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    /// Get a reference to the cli options's output.
    #[must_use]
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }

    /// Get a reference to the cli options's landscape.
    #[must_use]
    pub fn landscape(&self) -> bool {
        self.landscape
    }

    /// Get a reference to the cli options's background.
    #[must_use]
    pub fn background(&self) -> bool {
        self.background
    }

    /// Get a reference to the cli options's wait.
    #[must_use]
    pub fn wait(&self) -> Option<Duration> {
        self.wait
    }

    /// Get a reference to the cli options's header.
    #[must_use]
    pub fn header(&self) -> Option<&String> {
        self.header.as_ref()
    }

    /// Get a reference to the cli options's footer.
    #[must_use]
    pub fn footer(&self) -> Option<&String> {
        self.footer.as_ref()
    }

    /// Get a reference to the cli options's paper.
    #[must_use]
    pub fn paper(&self) -> Option<&PaperSize> {
        self.paper.as_ref()
    }

    /// Get a reference to the cli options's scale.
    #[must_use]
    pub fn scale(&self) -> Option<f32> {
        self.scale
    }

    /// Get a reference to the cli options's margin.
    #[must_use]
    pub fn margin(&self) -> Option<&Margin> {
        self.margin.as_ref()
    }

    /// Get a reference to the cli options's range.
    #[must_use]
    pub fn range(&self) -> Option<&String> {
        self.range.as_ref()
    }
}

impl From<&Options> for PrintToPdfOptions {
    fn from(opt: &Options) -> Self {
        PrintToPdfOptions {
            landscape: Some(opt.landscape()),
            display_header_footer: Some(opt.header().is_some() || opt.footer().is_some()),
            print_background: Some(opt.background()),
            scale: opt.scale(),
            paper_width: opt.paper().map(PaperSize::paper_width),
            paper_height: opt.paper().map(PaperSize::paper_height),
            margin_top: opt.margin().map(Margin::margin_top),
            margin_bottom: opt.margin().map(Margin::margin_bottom),
            margin_left: opt.margin().map(Margin::margin_left),
            margin_right: opt.margin().map(Margin::margin_right),
            page_ranges: opt.range().cloned(),
            ignore_invalid_page_ranges: None,
            header_template: opt.header().cloned(),
            footer_template: opt.footer().cloned(),
            prefer_css_page_size: None,
        }
    }
}

/// Paper size
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// The width
    #[must_use]
    pub fn paper_width(&self) -> f32 {
        match self {
            PaperSize::A0 => 33.1,
            PaperSize::A1 => 23.4,
            PaperSize::A2 => 16.5,
            PaperSize::A3 => 11.7,
            PaperSize::A4 => 8.27,
            PaperSize::A5 => 5.83,
            PaperSize::A6 => 4.13,
            PaperSize::Letter | PaperSize::Legal => 8.5,
            PaperSize::Tabloid => 11.0,
        }
    }

    /// The height
    #[must_use]
    pub fn paper_height(&self) -> f32 {
        match self {
            PaperSize::A0 => 46.8,
            PaperSize::A1 => 33.1,
            PaperSize::A2 => 23.4,
            PaperSize::A3 => 16.5,
            PaperSize::A4 => 11.7,
            PaperSize::A5 => 8.27,
            PaperSize::A6 => 5.83,
            PaperSize::Letter => 11.0,
            PaperSize::Legal | PaperSize::Tabloid => 17.0,
        }
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
            _ => Err(Error::InvalidPaperSize(s.to_string())),
        }
    }
}

/// Margin definition
#[derive(Debug, Clone, PartialEq)]
pub enum Margin {
    /// Same margin on all side
    All(f32),
    /// Same margin vertically and horizontally
    VerticalHorizontal(f32, f32),
    /// Custom margin for every side
    TopRightBottomLeft(f32, f32, f32, f32),
}

impl Margin {
    /// Margin top
    #[must_use]
    pub fn margin_top(&self) -> f32 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(f, _)
            | Margin::TopRightBottomLeft(f, _, _, _) => *f,
        }
    }
    /// Margin right
    #[must_use]
    pub fn margin_right(&self) -> f32 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(_, f)
            | Margin::TopRightBottomLeft(_, f, _, _) => *f,
        }
    }
    /// Margin bottom
    #[must_use]
    pub fn margin_bottom(&self) -> f32 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(f, _)
            | Margin::TopRightBottomLeft(_, _, f, _) => *f,
        }
    }
    /// Margin left
    #[must_use]
    pub fn margin_left(&self) -> f32 {
        match self {
            Margin::All(f)
            | Margin::VerticalHorizontal(_, f)
            | Margin::TopRightBottomLeft(_, _, _, f) => *f,
        }
    }
}

impl FromStr for Margin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(' ').filter(|s| !s.is_empty()).collect();
        match values.len() {
            1 => {
                let value = s.parse::<f32>().map_err(Error::InvalidMarginValue)?;
                Ok(Margin::All(value))
            }
            2 => {
                let v = values[0]
                    .parse::<f32>()
                    .map_err(Error::InvalidMarginValue)?;
                let h = values[1]
                    .parse::<f32>()
                    .map_err(Error::InvalidMarginValue)?;
                Ok(Margin::VerticalHorizontal(v, h))
            }
            4 => {
                let top = values[0]
                    .parse::<f32>()
                    .map_err(Error::InvalidMarginValue)?;
                let right = values[1]
                    .parse::<f32>()
                    .map_err(Error::InvalidMarginValue)?;
                let bottom = values[2]
                    .parse::<f32>()
                    .map_err(Error::InvalidMarginValue)?;
                let left = values[2]
                    .parse::<f32>()
                    .map_err(Error::InvalidMarginValue)?;
                Ok(Margin::TopRightBottomLeft(top, right, bottom, left))
            }
            _ => Err(Error::InvalidMarginDefinition(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("a0", PaperSize::A0)]
    #[test_case("A1", PaperSize::A1)]
    #[test_case("A2", PaperSize::A2)]
    #[test_case("A3", PaperSize::A3)]
    #[test_case("A4", PaperSize::A4)]
    #[test_case("A5", PaperSize::A5)]
    #[test_case("A6", PaperSize::A6)]
    #[test_case("letter", PaperSize::Letter)]
    #[test_case("Legal", PaperSize::Legal)]
    #[test_case("Tabloid", PaperSize::Tabloid)]
    fn should_parse_valid_paper_size(value: &str, expected: PaperSize) {
        let result = value.parse::<PaperSize>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn should_reject_invalid_paper_size() {
        let value = "plop";
        let result = value.parse::<PaperSize>();
        assert!(matches!(result, Err(Error::InvalidPaperSize(_))));
    }

    #[test]
    fn should_parse_valid_margin_all() {
        let value = "0.4";
        let result = value.parse::<Margin>();
        assert!(matches!(result, Ok(Margin::All(_))));
    }

    #[test]
    fn should_parse_valid_margin_vh() {
        let value = "0.4  0.7";
        let result = value.parse::<Margin>();
        assert!(matches!(result, Ok(Margin::VerticalHorizontal(_, _))));
    }

    #[test]
    fn should_parse_valid_margin_trbl() {
        let value = "0.2   0.3 0.4  0.5";
        let result = value.parse::<Margin>();
        assert!(matches!(result, Ok(Margin::TopRightBottomLeft(_, _, _, _))));
    }

    #[test]
    fn should_reject_invalid_margin() {
        let value = "0.2    0.3  0.4";
        let result = value.parse::<Margin>();
        assert!(matches!(result, Err(Error::InvalidMarginDefinition(_))));
    }

    #[test]
    fn should_reject_invalid_margin_value() {
        let value = "plop";
        let result = value.parse::<Margin>();
        assert!(matches!(result, Err(Error::InvalidMarginValue(_))));
    }
}
