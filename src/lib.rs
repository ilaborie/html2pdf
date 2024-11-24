#![forbid(unsafe_code)]
#![warn(clippy::perf)]
// #![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

use std::fmt::Debug;
use std::io::ErrorKind;
use std::num::ParseFloatError;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

use anyhow::Result;
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use tracing::{debug, info};

mod cli;

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

    /// Headless chrome issue
    #[display("Oops, an error occurs with headless chrome: {_0}")]
    HeadlessChromeError(anyhow::Error),

    /// I/O issue
    IoError(io::Error),
}

/// Run HTML to PDF with `headless_chrome`
///
/// # Errors
///
/// Could fail if there is I/O or Chrome headless issue
pub fn run(opt: &Options) -> Result<(), Error> {
    let input = dunce::canonicalize(opt.input())?;
    let output = if let Some(path) = opt.output() {
        path.clone()
    } else {
        let mut path = opt.input().clone();
        path.set_extension("pdf");
        path
    };

    html_to_pdf(input, output, opt.into(), opt.into(), opt.wait())?;

    Ok(())
}

/// Run HTML to PDF with `headless_chrome`
///
/// # Panics
/// Sorry, no error handling, just panic
///
/// # Errors
///
/// Could fail if there is I/O or Chrome headless issue
pub fn html_to_pdf<I, O>(
    input: I,
    output: O,
    pdf_options: PrintToPdfOptions,
    launch_options: LaunchOptions,
    wait: Option<Duration>,
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

    let local_pdf = print_to_pdf(&input, pdf_options, launch_options, wait)?;

    info!(?output, "Output file");
    fs::write(output.as_ref(), local_pdf)?;

    Ok(())
}

fn print_to_pdf(
    file_path: &str,
    pdf_options: PrintToPdfOptions,
    launch_options: LaunchOptions,
    wait: Option<Duration>,
) -> Result<Vec<u8>> {
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    let tab = tab.navigate_to(file_path)?.wait_until_navigated()?;

    if let Some(wait) = wait {
        info!(?wait, "Waiting before export to PDF");
        sleep(wait);
    }

    debug!(?pdf_options, "Using PDF options");
    let bytes = tab.print_to_pdf(Some(pdf_options))?;

    Ok(bytes)
}
