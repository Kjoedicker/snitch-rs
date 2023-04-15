use crate::{ 
    dir::find_project_filepaths, 
    statics::*,
    commands::commit, 
    trackers::{
        tracker::IssueTracker, 
        github::{init_instance}
    }, 
    config::{Config, self}
};
use std::fs::{ write, read_to_string };
use regex::Regex;
use threadpool::ThreadPool;
use lazy_static::lazy_static;

fn parse_issue_number_from_line(line: &str) -> &str {
    lazy_static!{
        static ref ISSUE_NUMBER_PATTERN: Regex = Regex::new("[0-9]+").unwrap();    
    }
    
    let issue_number = ISSUE_NUMBER_PATTERN.find(line).unwrap().as_str();

    issue_number
}

#[tokio::main]
async fn find_and_check_tracked_issues(config: Config, file: String) -> (String, Vec<String>) {
    let mut issues = Vec::new();

    let mut source_file: Vec<String> = file
        .split('\n')
        .map(String::from)
        .collect();

    let issue_tracker = init_instance(config);

    let mut updated_source_file: Vec<String> = vec![];
    while let Some(line) = source_file.pop() {

        if TAGGED_ISSUE_PATTERN.is_match(&line) {
            let issue_number = parse_issue_number_from_line(&line);

            let issue = issue_tracker.fetch_issue(issue_number).await;

            if issue.state != "closed" { 
                updated_source_file.push(line); 
            } else {
                issues.push(issue_number.to_string()); 
            }
        }
    }

    (updated_source_file.join("\n"), issues)
}

fn process_file(config: Config, filepath: String) {
    let file = read_to_string(&filepath).unwrap();

    let (
        source_file, 
        issues
     ) = find_and_check_tracked_issues(config, file);

    if issues.is_empty() { return };

    write(&filepath, source_file).unwrap();

    commit::commit_issues("Removed", &filepath, issues);
}

fn thread_files_for_processing(config: Config, filepaths: Vec<String>) {
    let pool = ThreadPool::new(config.total_threads.parse::<usize>().unwrap());

    for filepath in filepaths {
        let config = config.clone();

        let file_processing_thread = move || {
            process_file(config.clone(), filepath);
        };
    
        pool.execute(file_processing_thread)
    }

    println!(
        "Active count - {}\nQueued Count - {}", 
        pool.active_count(), 
        pool.queued_count()
    );

    pool.join();
}

pub fn audit() {
    let filepaths = find_project_filepaths();

    let config = config::init();

    thread_files_for_processing(config, filepaths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // TODO: add Tests!
        // audit();
    }
}