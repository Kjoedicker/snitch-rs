use crate::{
    commands::commit,
    config::{self, Config},
    dir::find_project_filepaths,
    statics::*,
    trackers::{github::init_instance, tracker::IssueTracker},
};
use std::fs::{read_to_string, write};
use threadpool::ThreadPool;

fn parse_context_from_line(line: &str) -> (String, String) {
    let lines: Vec<&str> = line.split(": ").collect();

    let prefix = String::from(lines[0]);
    let description = String::from(lines[1]);

    (prefix, description)
}

#[tokio::main]
async fn find_and_track_issues(config: Config, file: String) -> (String, Vec<String>) {
    let mut issues = Vec::new();

    let mut source_file: Vec<String> = file.split('\n').map(String::from).collect();

    let issue_tracker = init_instance(config);

    for line in source_file.iter_mut() {
        if UNTAGGED_ISSUE_PATTERN.is_match(line) {
            let (prefix, description) = parse_context_from_line(line);

            let issue = issue_tracker.create_issue(&description).await;

            *line = format!(
                "{}(#{}):{} - {}",
                prefix, &issue.number, description, &issue.html_url
            );

            issues.push(format!("{}", issue.number));
        }
    }

    (source_file.join("\n"), issues)
}

fn process_file(config: Config, filepath: String) {
    let file = read_to_string(&filepath).unwrap();

    let (source_file, issues) = find_and_track_issues(config, file);

    if issues.is_empty() {
        return;
    };

    write(&filepath, source_file).unwrap();

    commit::commit_issues("Added", &filepath, issues);
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

pub fn snitch() {
    let filepaths = find_project_filepaths();

    let config = config::init();

    thread_files_for_processing(config, filepaths);
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
            let reality = format!("{}: {}", prefix, description) == issue_line;

            assert_eq!(
                expectation, reality,
                "The prefix and description rebuilt, should match the original line"
            );
        }
    }

    mod find_and_track_issues {
        use crate::config::init;

        use super::*;

        #[test]
        fn matches_and_updates_issue_lines() {
            let file = String::from(
                "line 1\nline 2\nTODO: example todo\nline 4\nTODO: final example todo",
            );
            let config = init();

            let (updated_file, new_issues) = find_and_track_issues(config, file.clone());

            let reality = true;
            let expectation_1 = file.len() < updated_file.len();
            assert_eq!(expectation_1, reality);

            let expectation_2 = new_issues.len() == 2;
            assert_eq!(expectation_2, reality);
        }

        #[test]
        fn handles_empty_files_gracefully() {
            let file = String::from("");
            let config = init();

            let (updated_file, new_issues) = find_and_track_issues(config, file.clone());

            let reality = true;
            let expectation_1 = file.len() == updated_file.len();
            assert_eq!(
                expectation_1, reality,
                "The updated file should be exactly the same"
            );

            let expectation_2 = new_issues.is_empty();
            assert_eq!(expectation_2, reality, "There should be 0 matched issues");
        }
    }
}
