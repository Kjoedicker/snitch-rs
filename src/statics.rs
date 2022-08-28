use lazy_static::lazy_static;
use regex::{ Regex };

lazy_static! {
    pub static ref UNTAGGED_TODO_PATTERN: Regex = Regex::new(r"^(.*)TODO: (.*)").unwrap();
    pub static ref COMPLETED_TODO_PATTERN: Regex = Regex::new(r"DONE\(#(\d+)\):").unwrap();
}