use lazy_static::lazy_static;
use regex::{ Regex };

lazy_static! {
    // TODO: make matching patterns configurable
    pub static ref UNTAGGED_ISSUE_PATTERN: Regex = Regex::new(r"^(.*)TODO: (.*)").unwrap();
}