use std::env;

use html2pdf::{run, Options, Error};
use log::{debug, warn};
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let env_log = env::var("RUST_LOG");
    if let Ok(level) = env_log {
        pretty_env_logger::init();
        debug!("RUST_LOG is {}", level);
    } else {
        env::set_var("RUST_LOG", "info");
        pretty_env_logger::init();
        warn!("No RUST_LOG environment variable found, set log to 'info'")
    }

    let opt = Options::from_args();
    debug!("Options: {:#?}", opt);

    // Let's go
    run(opt)
}
