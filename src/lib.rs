//! Crate html2pdf

use std::fs;
use std::path::Path;

use headless_chrome::protocol::page::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use log::info;

mod cli;

pub use cli::*;

/// Run HTML to PDF with headless_chrome
///
/// # Panics
/// Sorry, no error handling, just panic
pub fn run(opt: CliOptions) {
    let input = fs::canonicalize(opt.input()).unwrap();
    let output = if let Some(out) = opt.output() {
        out.clone()
    } else {
        let mut out = opt.input().clone();
        out.set_extension("pdf");
        out
    };

    let pdf_options = PrintToPdfOptions {
        landscape: Some(opt.landscape()),
        display_header_footer: None,
        print_background: Some(opt.background()),
        scale: None,
        paper_width: None,
        paper_height: None,
        margin_top: None,
        margin_bottom: None,
        margin_left: None,
        margin_right: None,
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: None,
    };

    html_to_pdf(input, output, pdf_options);
}

/// Run HTML to PDF with headless_chrome
///
/// # Panics
/// Sorry, no error handling, just panic
pub fn html_to_pdf<I, O>(input: I, output: O, pdf_options: PrintToPdfOptions)
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let input = format!("file://{}", input.as_ref().as_os_str().to_str().unwrap());
    info!("Input file: {}", input);

    let local_pdf = print_to_pdf(&input, pdf_options);

    info!("Output file: {:?}", output.as_ref());
    fs::write(output, &local_pdf).unwrap();
}

fn print_to_pdf(file_path: &str, pdf_options: PrintToPdfOptions) -> Vec<u8> {
    let options = LaunchOptionsBuilder::default().build().unwrap();
    let browser = Browser::new(options).unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to(file_path)
        .unwrap()
        .wait_until_navigated()
        .unwrap()
        .print_to_pdf(Some(pdf_options))
        .unwrap()
}
