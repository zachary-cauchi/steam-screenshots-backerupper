pub mod app;
pub mod result;
pub mod steam;
pub mod u2c;

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
    /// The URL to the copyparty server. Should include the subdir to place screenshots in if applicable.
    #[arg(short = 'u', long)]
    server_url: String,
    /// The password authenticate with the server.
    #[arg(short = 'p', long)]
    server_password: String,
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt().with_max_level(args.level).init();

    debug!("Logging initialized");

    let app = App::new(args.server_url, args.server_password);

    app.run().unwrap();
}
