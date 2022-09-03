use clap::{ Parser, Subcommand };

#[derive(Debug, Subcommand)]
pub enum Mode {
    /// Find and report untracked issues
    Snitch,
    /// List existing issues
    Peek,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub mode: Mode,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
