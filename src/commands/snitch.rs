use crate::{ 
    dir::find_project_filepaths, 
    statics::*,
    trackers::github::{ create_issue }
};
use std::{
    fs::{ write, read_to_string },
    process::{ Command },
    sync::{ Arc, Mutex }
};
use threadpool::ThreadPool;

fn format_issues(issues: Vec<String>) -> String {

    let concated_issues = format!(
        "#{}", 
        issues.join(", #")
    );

    concated_issues
}

fn format_commit_message(issues: &String) -> String {

    let base_message = format!(
        "Adding {}", 
        match issues.len() > 1 {
            true => "issues: ",
            _ => "issue: "
        }
    );

    let commit_message = format!(
        "{}{}",
        base_message, 
        issues
    );

    commit_message
}

fn commit_file(filepath: &str, commit_message: String) {

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .arg("--include")
        .arg(filepath)
        .output()
        .expect(
            &format!(
                "Failed to commit '{}'\n for: {}`", 
                commit_message,
                filepath 
            )
        ).stdout;
}

fn commit_reported_issues(filepath: &str, issues: Vec<String>) {

    let formatted_issues = format_issues(issues);
    let commit_message= format_commit_message(&formatted_issues);

    commit_file(&filepath, commit_message);

    println!("[COMMITED] issues: {}", formatted_issues);
}

fn parse_context_from_line(line: &str) -> (String, String) {
    let lines: Vec<&str> = line.split(':').collect();

    let prefix = String::from(lines[0]);
    let description = String::from(lines[1]);
    
    (prefix, description)
}

fn process_file(file: String) -> (Vec<String>, Vec<String>) {
    let mut issues = Vec::new();

    let mut source_file: Vec<String> = file
        .split("\n")
        .map(|x| String::from(x))
        .collect();

    for (line_number, line) in source_file.clone().iter().enumerate() {
    
        if UNTAGGED_ISSUE_PATTERN.is_match(line) {

            let (
                prefix,
                description
            ) = parse_context_from_line(&line);

            let issue = create_issue(&description, "").unwrap();

            source_file[line_number] = format!("{}(#{}):{}", prefix, &issue.number, description);
    
            issues.push(format!("{}", issue.number));
        }
    }

    (source_file, issues)

}

fn process_filepaths(filepaths: Vec<String>) {

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

            if issues.len() > 0 {
                write(&filepath, source_file.join("\n")).unwrap();

                // This stops a race condition when `commit_reported_issues` 
                // is called at the same time across threads 
                let _lock_power_to_commit = power_to_commit.lock().unwrap();
    
                commit_reported_issues(&filepath, issues);
            }
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

    process_filepaths(filepaths);
}

#[cfg(test)]
mod snitch_tests {
    use super::*;

    fn str_to_string(val: &str) -> String {
        String::from(val)
    }

    #[test]
    fn test_format_issues() {
        let test_issues: Vec<String> = vec![
            str_to_string("1"),
            str_to_string("2"),
            str_to_string("3")
        ];

        let formatted_issues = format_issues(test_issues);

        let expectation = true;
        let reality = formatted_issues == "#1, #2, #3";

        assert_eq!(expectation, reality, "Issues should be formatted properly");
    }

    #[test]
    fn test_format_commit_message() {
        let test_issues: Vec<String> = vec![
            str_to_string("1"),
            str_to_string("2"),
            str_to_string("3")
        ];

        let formatted_issues = format_issues(test_issues);

        let commit_message = format_commit_message(&formatted_issues);

        let expectation = true;
        let reality = commit_message == "Adding issues: #1, #2, #3";

        assert_eq!(expectation, reality);
    }

    // #[test]
    // fn test_commit_file() {

    // }

    // #[test]
    // fn test_commit_reported_issues() {

    // }

    #[test]
    fn test_parse_context_from_line() {
        let issue_line = "TODO: figure out more convenient macros for testing";
        
        let (prefix, description) = parse_context_from_line(issue_line);

        let expectation = true;
        let reality = format!("{}:{}", prefix, description) == issue_line;

        assert_eq!(expectation, reality);
    }

    #[test]
    fn test_process_file() {
        let file = String::from("line 1\nline 2\nTODO: example todo\nline 4\nTODO: final example todo");
        let (updated_file, new_issues) = process_file(file);

        let reality= true;
        let expectation_1 = updated_file.len() == 5;
        assert_eq!(expectation_1, reality);

        let expectation_2 = new_issues.len() == 2;
        assert_eq!(expectation_2, reality);
    }

    // #[test]
    // fn test_process_filepaths() {

    // }

}