use std::env;

use html2pdf::{run, CliOptions};
use log::{debug, warn};
use structopt::StructOpt;

fn main() {
    let env_log = env::var("RUST_LOG");
    if let Ok(level) = env_log {
        pretty_env_logger::init();
        debug!("RUST_LOG is {}", level);
    } else {
        env::set_var("RUST_LOG", "info");
        pretty_env_logger::init();
        warn!("No RUST_LOG environment variable found, set log to 'info'")
    }

    let opt = CliOptions::from_args();
    debug!("CliOptions: {:#?}", opt);

    // Let's go
    run(opt);
}
