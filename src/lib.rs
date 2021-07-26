//! html2pdf
//! Generate a PDF from a local HTML file using a headless chrome

use std::fmt::Debug;
use std::fs;
use std::num::ParseFloatError;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

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
    #[error("Oops, an error occurs with IO: {0}")]
    IoError(String),
}

/// Run HTML to PDF with headless_chrome
pub fn run(opt: CliOptions) -> Result<(), Error> {
    let input =
        fs::canonicalize(opt.input()).map_err(|err| Error::IoError(format!("{:?}", err)))?;
    let output = if let Some(out) = opt.output() {
        out.clone()
    } else {
        let mut out = opt.input().clone();
        out.set_extension("pdf");
        out
    };

    let pdf_options = PrintToPdfOptions {
        landscape: Some(opt.landscape()),
        display_header_footer: Some(opt.header().is_some() || opt.footer().is_some()),
        print_background: Some(opt.background()),
        scale: opt.scale(),
        paper_width: opt.paper().map(|ps| ps.paper_width()),
        paper_height: opt.paper().map(|ps| ps.paper_height()),
        margin_top: opt.margin().map(|m| m.margin_top()),
        margin_bottom: opt.margin().map(|m| m.margin_bottom()),
        margin_left: opt.margin().map(|m| m.margin_left()),
        margin_right: opt.margin().map(|m| m.margin_right()),
        page_ranges: opt.range().cloned(),
        ignore_invalid_page_ranges: None,
        header_template: opt.header().cloned(),
        footer_template: opt.footer().cloned(),
        prefer_css_page_size: None,
    };

    html_to_pdf(input, output, pdf_options, opt.wait())?;

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
        .ok_or_else(|| Error::IoError(format!("Oops, input file invalid: {:?}", input)))?;
    let input = format!("file://{}", os);
    info!("Input file: {}", input);

    let local_pdf = print_to_pdf(&input, pdf_options, wait)?;

    info!("Output file: {:?}", output.as_ref());
    fs::write(output.as_ref(), &local_pdf)
        .map_err(|err| Error::IoError(format!("Fail to write file {:?} : {:?}", output, err)))?;

    Ok(())
}

fn print_to_pdf(
    file_path: &str,
    pdf_options: PrintToPdfOptions,
    wait: Option<Duration>,
) -> Result<Vec<u8>, Error> {
    let options = LaunchOptionsBuilder::default()
        .build()
        .map_err(Error::HeadlessChromeError)?;
    let browser =
        Browser::new(options).map_err(|err| Error::HeadlessChromeError(err.to_string()))?;
    let tab = browser
        .wait_for_initial_tab()
        .map_err(|err| Error::HeadlessChromeError(err.to_string()))?;

    let tab = tab
        .navigate_to(file_path)
        .map_err(|err| Error::HeadlessChromeError(err.to_string()))?
        .wait_until_navigated()
        .map_err(|err| Error::HeadlessChromeError(err.to_string()))?;

    if let Some(wait) = wait {
        info!("Waiting {} before export to PDF", format_duration(wait));
        sleep(wait);
    }

    debug!("Using PDF options: {:?}", pdf_options);
    let bytes = tab
        .print_to_pdf(Some(pdf_options))
        .map_err(|err| Error::HeadlessChromeError(err.to_string()))?;

    Ok(bytes)
}
