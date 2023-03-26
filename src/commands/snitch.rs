use crate::{ 
    dir::find_project_filepaths, 
    statics::*,
    trackers::github::{ create_issue },
    commands::commit
};
use std::{
    fs::{ write, read_to_string },
    sync::{ Arc, Mutex }
};
use threadpool::ThreadPool;

fn parse_context_from_line(line: &str) -> (String, String) {
    let lines: Vec<&str> = line.split(':').collect();

    let prefix = String::from(lines[0]);
    let description = String::from(lines[1]);
    
    (prefix, description)
}

#[tokio::main]
async fn process_file(file: String) -> (String, Vec<String>) {
    let mut issues = Vec::new();

    let mut source_file: Vec<String> = file
        .split('\n')
        .map(String::from)
        .collect();

    for (line_number, line) in source_file.clone().iter().enumerate() {
    
        if UNTAGGED_ISSUE_PATTERN.is_match(line) {

            let (
                prefix,
                description
            ) = parse_context_from_line(line);

            let issue = create_issue(&description, None).await;

            source_file[line_number] = format!("{}(#{}):{} - {}", prefix, &issue.number, description, &issue.html_url);
    
            issues.push(format!("{}", issue.number));
        }
    }

    (source_file.join("\n"), issues)
}

fn queue_files_for_processing(filepaths: Vec<String>) {

    let pool = ThreadPool::new(CONFIG.total_threads);
    let commit_action =  Arc::new(Mutex::new(true));

    for filepath in filepaths {
        let power_to_commit = Arc::clone(&commit_action);

        let thread_file_processing = move || {

            let file = read_to_string(&filepath).unwrap();

            let (
                source_file, 
                issues
             ) = process_file(file);

            if issues.is_empty() { return };

            write(&filepath, source_file).unwrap();

            // This stops a race condition when `commit_reported_issues` 
            // is called at the same time across threads 
            let _lock_power_to_commit = power_to_commit.lock().unwrap();

            commit::commit_reported_issues(&filepath, issues);
        };
    
        pool.execute(thread_file_processing)
    }

    println!(
        "Active count - {}\nQueued Count - {}", 
        pool.active_count(), 
        pool.queued_count()
    );

    pool.join();
}

pub fn snitch() {
    let filepaths = find_project_filepaths();

    queue_files_for_processing(filepaths);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_context_from_line {
        use super::*;

        #[test]
        fn parses_prefix_and_description() {
            let issue_line = "TODO: figure out more convenient macros for testing";
            
            let (prefix, description) = parse_context_from_line(issue_line);
            
            let expectation = true;
            let reality = format!("{}:{}", prefix, description) == issue_line;
            
            assert_eq!(expectation, reality, "The prefix and description rebuilt, should match the original line");
        }
    }

    mod process_file {
        use super::*;

        #[test]
        fn matches_and_updates_issue_lines() {
            let file = String::from("line 1\nline 2\nTODO: example todo\nline 4\nTODO: final example todo");
            let (updated_file, new_issues) = process_file(file.clone());

            let reality= true;
            let expectation_1 = file.len() < updated_file.len();
            assert_eq!(expectation_1, reality);

            let expectation_2 = new_issues.len() == 2;
            assert_eq!(expectation_2, reality);
        }

        #[test]
        fn handles_empty_files_gracefully() {
            let file = String::from("");
            let (updated_file, new_issues) = process_file(file.clone());

            let reality= true;
            let expectation_1 = file.len() == updated_file.len();
            assert_eq!(expectation_1, reality, "The updated file should be exactly the same");

            let expectation_2 = new_issues.len() == 0;
            assert_eq!(expectation_2, reality, "There should be 0 matched issues");
        }
    }
}