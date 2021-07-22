use std::path::PathBuf;
use std::time::Duration;

use humantime::parse_duration;
use structopt::StructOpt;

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
    // TODO: allow to configure other PrintToPdfOptions options
    // display_header_footer: Option<bool>,

    // scale: Option<f32>,

    // paper_width: Option<f32>,
    // paper_height: Option<f32>,

    // margin_top: Option<f32>,
    // margin_bottom: Option<f32>,
    // margin_left: Option<f32>,
    // margin_right: Option<f32>,

    // page_ranges: Option<String>,
    // ignore_invalid_page_ranges: Option<String>,

    // header_template: Option<String>,
    // footer_template: Option<String>,

    // prefer_css_page_size: Option<bool>
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
}
