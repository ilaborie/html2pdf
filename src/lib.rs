#![forbid(unsafe_code)]
#![warn(clippy::perf)]
// #![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

use std::error::Error as StdError;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::num::ParseFloatError;
use std::path::Path;
use std::{fs, io};

use tracing::info;

mod cli;
mod render;

pub use cli::*;

/// The html2pdf Error
#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
pub enum Error {
    /// Invalid paper size
    #[display(
        "Invalid paper size {size}, expected a value in A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6"
    )]
    #[from(ignore)]
    InvalidPaperSize {
        /// The invalid size
        size: String,
    },

    /// Invalid margin definition
    #[display("Invalid margin definition, expected 1, 2, or 4 value, got {margin}")]
    #[from(ignore)]
    InvalidMarginDefinition {
        /// the invalid margin
        margin: String,
    },

    /// Invalid margin value
    #[display("Invalid margin value: {_0}")]
    InvalidMarginValue(ParseFloatError),

    /// Invalid page-ready milestone
    #[display("Invalid wait-for value {value}, expected one of navigation, load, network-idle")]
    #[from(ignore)]
    InvalidWaitFor {
        /// The invalid value
        value: String,
    },

    /// Headless browser issue
    ///
    /// The underlying cause is kept boxed on purpose, so the browser crate stays an
    /// implementation detail rather than part of this crate's public API.
    #[display("Oops, an error occurs with the headless browser: {_0}")]
    #[from(ignore)]
    Browser(Box<dyn StdError + Send + Sync>),

    /// I/O issue
    IoError(io::Error),
}

impl Error {
    /// Wrap any browser-backend failure into [`Error::Browser`].
    pub(crate) fn browser<E>(err: E) -> Self
    where
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        Self::Browser(err.into())
    }
}

/// Run HTML to PDF with a headless browser
///
/// # Errors
///
/// Could fail if there is I/O or Chrome headless issue
pub async fn run(opt: &Options) -> Result<(), Error> {
    let input = dunce::canonicalize(&opt.input)?;
    let output = opt.output.clone().unwrap_or_else(|| {
        let mut path = opt.input.clone();
        path.set_extension("pdf");
        path
    });

    html_to_pdf(input, output, opt.into(), opt.into()).await
}

/// Run HTML to PDF with a headless browser
///
/// # Errors
///
/// Could fail if there is I/O or Chrome headless issue
pub async fn html_to_pdf<I, O>(
    input: I,
    output: O,
    pdf_options: PdfOptions,
    browser_options: BrowserOptions,
) -> Result<(), Error>
where
    I: AsRef<Path> + Debug,
    O: AsRef<Path> + Debug,
{
    let os = input
        .as_ref()
        .as_os_str()
        .to_str()
        .ok_or_else(|| io::Error::from(ErrorKind::InvalidInput))?;
    let input = format!("file://{os}");
    info!(%input, "Input file");

    let local_pdf = render::print_to_pdf(&input, &pdf_options, &browser_options).await?;

    info!(?output, "Output file");
    fs::write(output.as_ref(), local_pdf)?;

    Ok(())
}
