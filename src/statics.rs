use lazy_static::lazy_static;
use regex::{ Regex };

use crate::config::*;

lazy_static! {
    //TODO: make this pattern configurable and more dynamic
    pub static ref UNTAGGED_ISSUE_PATTERN: Regex = Regex::new(r"^(.*)TODO: (.*)").unwrap();
    pub static ref COMPLETED_ISSUE_PATTERN: Regex = Regex::new(r"DONE\(#(\d+)\):").unwrap();

    #[derive(Debug)]
    pub static ref CONFIG: Config = init();
}