use crate::{ 
    dir::find_project_filepaths, 
    statics::*,
    trackers::github::{ create_issue }
};
use std::{
    fs::{ write, read_to_string },
    process::{ Command }
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
        "{} {}",
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

fn process_file(filepath: &str) {
    let file = read_to_string(filepath).unwrap();

    let mut issues = Vec::new();

    let mut original_lines: Vec<String> = file
        .split("\n")
        .map(|x| String::from(x))
        .collect();

    let lines= original_lines.clone();

    for (line_number, line) in  lines.iter().enumerate(){
        if match_line(line) == "untagged" {
            let (prefix, description) = 
                parse_context_from_line(&line);

            let issue = create_issue(&description, "").unwrap();

            original_lines[line_number] = format!("{}(#{}):{}", prefix, &issue.number, description);
    
            issues.push(format!("{}", issue.number));
        }
    }

    if issues.len() > 0 {
        write(filepath, original_lines.join("\n")).unwrap();
        // Add mutex lock for committing to stop race condition where a commit takes precedence
        commit_reported_issues(filepath, issues);
    }

}

fn process_files(filepaths: Vec<String>) {

    let pool = ThreadPool::new(CONFIG.total_threads);

    for filepath in filepaths {

        let thread_file_processing = move || {
            process_file(&filepath);
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
