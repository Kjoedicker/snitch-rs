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

fn commit_reported_issues(filepath: &str, issues: Vec<String>) {

    let base_message = format!(
        "Adding {}", 
        match issues.len() > 1 {
            true => "issues: ",
            _ => "issue: "
        }
    );

    let concated_issues = format!(
        "#{}", 
        issues.join(", #")
    );

    let commit_message = format!(
        "{}{}",
        base_message, 
        concated_issues
    );

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .arg("--include")
        .arg(filepath)
        .output()
        .expect(
            &format!(
                "Failed to commit issue {}, {}`", 
                concated_issues,
                filepath 
            )
        ).stdout;

    println!("[COMMITED] issues: {}", concated_issues);
    
}


fn match_line(line: &str) -> &str {
    let mut pattern = "";
        
    if UNTAGGED_ISSUE_PATTERN.is_match(line) {
        pattern = "untagged";
    }

    pattern
}

fn parse_context_from_line(line: &str) -> (String, String) {
    let lines: Vec<&str> = line.split(':').collect();

    let prefix = String::from(lines[0]);
    let description = String::from(lines[1]);
    
    (prefix, description)
}

fn process_file(filepath: &str) -> (Vec<String>, Vec<String>) {
    let file = read_to_string(filepath).unwrap();

    let mut issues = Vec::new();

    let mut source_file: Vec<String> = file
        .split("\n")
        .map(|x| String::from(x))
        .collect();

    let file_lines= source_file.clone();

    for (line_number, line) in  file_lines.iter().enumerate(){
        if match_line(line) == "untagged" {
            let (prefix, description) = 
                parse_context_from_line(&line);

            let issue = create_issue(&description, "").unwrap();

            source_file[line_number] = format!("{}(#{}):{}", prefix, &issue.number, description);
    
            issues.push(format!("{}", issue.number));
        }
    }

    (source_file, issues)

}

fn process_files(filepaths: Vec<String>) {

    let pool = ThreadPool::new(CONFIG.total_threads);
    let commit_action =  Arc::new(Mutex::new(true));

    for filepath in filepaths {
        let power_to_commit = Arc::clone(&commit_action);

        let thread_file_processing = move || {
            let (source_file, issues) = process_file(&filepath);

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

    process_files(filepaths);
}
