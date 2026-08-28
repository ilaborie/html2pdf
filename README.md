# html2pdf

[![Docs](https://docs.rs/html2pdf/badge.svg)](https://docs.rs/html2pdf)
[![Crates.io](https://img.shields.io/crates/v/html2pdf.svg?maxAge=2592000)](https://crates.io/crates/html2pdf)

Just a CLI over the [`headless_chrome`](https://crates.io/crates/headless_chrome) crate to create PDF.

## Install

Need the Rust toolchain: <https://rustup.rs/>.

```shell
cargo install html2pdf
```

## Usage

```shell
html2pdf path/to/file.html
```

To remove logs, set the env var `RUST_LOG` to `none` :

```shell
RUST_LOG="none" html2pdf path/to/file.html
```

## Options

Just run `html2pdf --help` :

```shell
Generate a PDF from a local HTML file using a headless chrome

Usage: html2pdf [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input HTML file

Options:
  -o, --output <OUTPUT>      Output file. By default, just change the input extension to PDF
      --landscape            Use landscape mode
      --background           Allow print background
      --wait <WAIT>          Time to wait in ms before printing. Examples: 150ms, 10s
      --wait-for <WAIT_FOR>  When to consider the page ready for printing. Supported values:
                             navigation, load, network-idle [default: navigation]
      --header <HEADER>      HTML template for the print header
      --footer <FOOTER>      HTML template for the print footer
      --paper <PAPER>        Paper size. Supported values: A4, Letter, Legal, A3, Tabloid, A2, A1,
                             A0, A5, A6
      --scale <SCALE>        Scale, default to 1.0
      --range <RANGE>        Paper ranges to print, e.g. '1-5, 8, 11-13'
      --margin <MARGIN>      Margin in inches. '0.4' applies to all sides, '0.4 0.4' is
                             top/bottom then left/right, '0.4 0.4 0.4 0.4' is top, right,
                             bottom, left
      --disable-sandbox      Disable Chrome sandbox. Not recommended, unless running on docker
  -h, --help                 Print help
  -V, --version              Print version
```

### Waiting for the page to be ready

By default the PDF is printed as soon as navigation completes. When the page pulls in web fonts,
images or data, that can be too early. `--wait-for` waits for a browser lifecycle milestone
instead:

| Value | Waits until |
|---|---|
| `navigation` | navigation completes (default, the historical behaviour) |
| `load` | the `load` event, i.e. sub-resources have been fetched |
| `network-idle` | the network has gone quiet, i.e. fonts, images and XHR have settled |

If the milestone is never reached (a page that polls forever, say), `html2pdf` warns and prints
anyway rather than hanging. `--wait` is still available and applies *after* `--wait-for` settles.

## Library

`html2pdf` is also a library. The API is async and runs on [tokio]:

```rust,no_run
use html2pdf::{html_to_pdf, BrowserOptions, PaperSize, PdfOptions, WaitFor};

#[tokio::main]
async fn main() -> Result<(), html2pdf::Error> {
    html_to_pdf(
        "input.html",
        "output.pdf",
        PdfOptions {
            paper: Some(PaperSize::A4),
            print_background: true,
            ..PdfOptions::default()
        },
        BrowserOptions {
            wait_for: WaitFor::NetworkIdle,
            ..BrowserOptions::default()
        },
    )
    .await
}
```

[tokio]: https://tokio.rs
