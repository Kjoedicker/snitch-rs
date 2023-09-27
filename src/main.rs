use std::time::Instant;

mod cli;
mod commands;
mod config;
mod dir;
mod helpers;
mod statics;
mod trackers;

use cli::*;
use commands::*;

#[tokio::main]
async fn main() {
    // TODO: should we abstract this out ?
    let now = Instant::now();

    let cli = cli::parse_args();

    match cli.mode {
        Mode::Peek => {
            peek().await;
        }
        Mode::Snitch => snitch(),
        Mode::Audit => audit(),
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
