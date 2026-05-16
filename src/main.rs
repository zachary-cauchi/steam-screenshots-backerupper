pub mod app;
pub mod result;
pub mod steam;

use clap::Parser;
use tracing::debug;

use crate::app::App;

#[derive(Parser, Debug)]
#[clap(author = "Zachary Cauchi", version, about)]
/// Application configuration
struct Args {
    /// Maximum log level. Available options:
    /// ERROR, WARN, INFO, DEBUG, TRACE.
    #[arg(short = 'l', default_value_t = tracing::Level::INFO)]
    level: tracing::Level,
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt().with_max_level(args.level).init();

    debug!("Logging initialized");

    let app = App {};

    app.run().unwrap();
}
