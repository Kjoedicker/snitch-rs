mod cli;
mod config;
mod commands;
mod db;
mod dir;
mod statics;
mod issue;

use cli::*;
use commands::*;

fn main() {
    let cli = cli::parse_args();

    match cli.mode {
        Mode::Peek => {
            peek()
        }
        Mode::Snitch => {
            snitch()
        }
    }
}
