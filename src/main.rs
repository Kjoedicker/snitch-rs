use std::time::Instant;

mod cli;
mod config;
mod commands;
mod dir;
mod helpers;
mod statics;
mod trackers;

use cli::*;
use commands::*;

fn main() {
        // TODO: should we abstract this out ?
        let now = Instant::now();
        
        let cli = cli::parse_args();
        
        match cli.mode {
            Mode::Peek => {
                peek()
            }
            Mode::Snitch => {
                snitch()
            }
        }
        
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
}
