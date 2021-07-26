use core::f32;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use humantime::parse_duration;
use structopt::StructOpt;

use crate::Error;

/// Generate a PDF from a local HTML file using a headless chrome
#[derive(Debug, StructOpt)]
pub struct CliOptions {
    /// Input HTML file.
    #[structopt()]
    input: PathBuf,

    /// Output file.
    /// By default, just change the input extension to PDF
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Use landscape mode.
    #[structopt(long)]
    landscape: bool,

    /// Allow print background.
    #[structopt(long)]
    background: bool,

    /// Time to wait in ms before printing.
    /// Examples: 150ms, 10s
    #[structopt(long, parse(try_from_str = parse_duration))]
    wait: Option<Duration>,

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
    header: Option<String>,

    /// HTML template for the print footer.
    /// Should use the same format as the headerTemplate.
    #[structopt(long)]
    footer: Option<String>,

    /// Paper size
    /// Supported values: A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6
    #[structopt(long)]
    paper: Option<PaperSize>,

    /// Scale
    #[structopt(long)]
    scale: Option<f32>,

    /// Paper ranges to print
    /// e.g., '1-5, 8, 11-13'
    #[structopt(long)]
    range: Option<String>,

    /// Margin in inches
    /// You can define margin like this:
    /// '0.4' the value is applied for all side,
    /// '0.4 0.4' : first value is applied for top and bottom, second for left and right,
    /// '0.4 0.4 0.4 0.4' : first value is applied for top then, right, then bottom, and last for left
    #[structopt(long)]
    margin: Option<Margin>,
}

impl CliOptions {
    /// Get a reference to the cli options's input.
    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    /// Get a reference to the cli options's output.
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }

    /// Get a reference to the cli options's landscape.
    pub fn landscape(&self) -> bool {
        self.landscape
    }

    /// Get a reference to the cli options's background.
    pub fn background(&self) -> bool {
        self.background
    }

    /// Get a reference to the cli options's wait.
    pub fn wait(&self) -> Option<Duration> {
        self.wait
    }

    /// Get a reference to the cli options's header.
    pub fn header(&self) -> Option<&String> {
        self.header.as_ref()
    }

    /// Get a reference to the cli options's footer.
    pub fn footer(&self) -> Option<&String> {
        self.footer.as_ref()
    }

    /// Get a reference to the cli options's paper.
    pub fn paper(&self) -> Option<&PaperSize> {
        self.paper.as_ref()
    }

    /// Get a reference to the cli options's scale.
    pub fn scale(&self) -> Option<f32> {
        self.scale
    }

    /// Get a reference to the cli options's margin.
    pub fn margin(&self) -> Option<&Margin> {
        self.margin.as_ref()
    }

    /// Get a reference to the cli options's range.
    pub fn range(&self) -> Option<&String> {
        self.range.as_ref()
    }
}

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
    pub fn paper_width(&self) -> f32 {
        match self {
            PaperSize::A0 => 33.1,
            PaperSize::A1 => 23.4,
            PaperSize::A2 => 16.5,
            PaperSize::A3 => 11.7,
            PaperSize::A4 => 8.27,
            PaperSize::A5 => 5.83,
            PaperSize::A6 => 4.13,
            PaperSize::Letter => 8.5,
            PaperSize::Legal => 8.5,
            PaperSize::Tabloid => 11.0,
        }
    }

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
            PaperSize::Legal => 17.0,
            PaperSize::Tabloid => 17.0,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Margin {
    All(f32),
    VerticalHorizontal(f32, f32),
    TopRightBottomLeft(f32, f32, f32, f32),
}

impl Margin {
    pub fn margin_top(&self) -> f32 {
        match self {
            Margin::All(f) => *f,
            Margin::VerticalHorizontal(f, _) => *f,
            Margin::TopRightBottomLeft(f, _, _, _) => *f,
        }
    }
    pub fn margin_right(&self) -> f32 {
        match self {
            Margin::All(f) => *f,
            Margin::VerticalHorizontal(_, f) => *f,
            Margin::TopRightBottomLeft(_, f, _, _) => *f,
        }
    }
    pub fn margin_bottom(&self) -> f32 {
        match self {
            Margin::All(f) => *f,
            Margin::VerticalHorizontal(f, _) => *f,
            Margin::TopRightBottomLeft(_, _, f, _) => *f,
        }
    }
    pub fn margin_left(&self) -> f32 {
        match self {
            Margin::All(f) => *f,
            Margin::VerticalHorizontal(_, f) => *f,
            Margin::TopRightBottomLeft(_, _, _, f) => *f,
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
