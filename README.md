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
html2pdf 0.4.0
Generate a PDF from a local HTML file using a headless chrome

USAGE:
    html2pdf [FLAGS] [OPTIONS] <input>

FLAGS:
        --background       Allow print background
    -h, --help             Prints help information
        --landscape        Use landscape mode
        --disable-sandbox  Disable Chrome sandbox. Not recommended, unless running on docker

    -V, --version          Prints version information

OPTIONS:
        --footer <footer>  HTML template for the print footer. Should use the same format as the headerTemplate
        --header <header>  HTML template for the print header. Should be valid HTML markup with following classes used
                           to inject printing values into them: date for formatted print date, title for document
                           title, url for document location, pageNumber for current page number, totalPages for total
                           pages in the document. For example, `<span class=title></span>` would generate span
                           containing the title
        --margin <margin>  Margin in inches You can define margin like this: '0.4' the value is applied for all side,
                                  '0.4 0.4' : first value is applied for top and bottom, second for left and right, '0.4 0.4
                                  0.4 0.4' : first value is applied for top then, right, then bottom, and last for left
    -o, --output <output>  Output file. By default, just change the input extension to PDF
        --paper <paper>    Paper size. Supported values: A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6
        --range <range>    Paper ranges to print, e.g. '1-5, 8, 11-13'
        --scale <scale>    Scale, default to 1.0
        --wait <wait>      Time to wait in ms before printing. Examples: 150ms, 10s

ARGS:
    <input>    Input HTML file
```
