#![doc = include_str!("../README.md")]

use std::fmt::Debug;
use std::io::ErrorKind;
use std::num::ParseFloatError;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

use headless_chrome::protocol::page::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use humantime::format_duration;
use log::{debug, info};
use thiserror::Error;

mod cli;

pub use cli::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "Invalid paper size {0}, expected a value in A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6"
    )]
    InvalidPaperSize(String),
    #[error("Invalid margin definition, expected 1, 2, or 4 value, got {0}")]
    InvalidMarginDefinition(String),
    #[error("Invalid margin value: {0}")]
    InvalidMarginValue(ParseFloatError),
    #[error("Oops, an error occurs with headless chrome: {0}")]
    HeadlessChromeError(String),
    #[error("Oops, an error occurs with IO")]
    IoError { source: io::Error },
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Error::IoError { source }
    }
}

impl From<ParseFloatError> for Error {
    fn from(source: ParseFloatError) -> Self {
        Error::InvalidMarginValue(source)
    }
}

impl From<failure::Error> for Error {
    fn from(source: failure::Error) -> Self {
        Error::HeadlessChromeError(source.to_string())
    }
}

/// Run HTML to PDF with headless_chrome
pub fn run(opt: CliOptions) -> Result<(), Error> {
    let input = dunce::canonicalize(opt.input())?;
    let output = if let Some(out) = opt.output() {
        out.clone()
    } else {
        let mut out = opt.input().clone();
        out.set_extension("pdf");
        out
    };

    html_to_pdf(input, output, (&opt).into(), opt.wait())?;

    Ok(())
}

/// Run HTML to PDF with headless_chrome
///
/// # Panics
/// Sorry, no error handling, just panic
pub fn html_to_pdf<I, O>(
    input: I,
    output: O,
    pdf_options: PrintToPdfOptions,
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
    let input = format!("file://{}", os);
    info!("Input file: {}", input);

    let local_pdf = print_to_pdf(&input, pdf_options, wait)?;

    info!("Output file: {:?}", output.as_ref());
    fs::write(output.as_ref(), &local_pdf)?;

    Ok(())
}

fn print_to_pdf(
    file_path: &str,
    pdf_options: PrintToPdfOptions,
    wait: Option<Duration>,
) -> Result<Vec<u8>, failure::Error> {
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Default should not panic");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    let tab = tab.navigate_to(file_path)?.wait_until_navigated()?;

    if let Some(wait) = wait {
        info!("Waiting {} before export to PDF", format_duration(wait));
        sleep(wait);
    }

    debug!("Using PDF options: {:?}", pdf_options);
    let bytes = tab.print_to_pdf(Some(pdf_options))?;

    Ok(bytes)
}
