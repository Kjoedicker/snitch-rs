use lazy_static::lazy_static;
use regex::{ Regex };

use crate::config::*;

lazy_static! {
    pub static ref UNTAGGED_TODO_PATTERN: Regex = Regex::new(r"^(.*)TODO: (.*)").unwrap();
    pub static ref COMPLETED_TODO_PATTERN: Regex = Regex::new(r"DONE\(#(\d+)\):").unwrap();

    #[derive(Debug)]
    pub static ref CONFIG: Config = init();
}