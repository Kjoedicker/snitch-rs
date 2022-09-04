use crate::{ 
    dir::find_project_filepaths, 
    statics::*,
    trackers::github::{Issue, fetch_issues, create_issue}
};
use std::{
    fs::{ write, read_to_string }
};
use threadpool::ThreadPool;

fn update_file(file: &String, file_data: String) {
    write(file, file_data).unwrap_or_else(|err| {
        println!("{file} - Error writing to file: {err}")
    });
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

fn process_file(filepath: &str) -> String {
    let file = read_to_string(filepath).unwrap();

    let mut updated_file_data = String::new();

    for line in file.lines() {
        match match_line(line) {
            "untagged" => {
                let (prefix, description) = 
                    parse_context_from_line(&line);

                let issue = create_issue(&description, "").unwrap();

                let issue_line = String::from(
                    format!("{}(#{}):{}\n", prefix, issue.number, description)
                );

                updated_file_data.push_str(&issue_line);
            },
            _ => {
                updated_file_data.push_str(&format!("{}\n", line))
            }
        }
    }

    updated_file_data
}

fn process_files(filepaths: Vec<String>) {

    let pool = ThreadPool::new(CONFIG.total_threads);

    for filepath in filepaths {

        let thread_file_processing = move || {
            let updated_file_data = process_file(&filepath);

            update_file(&filepath, updated_file_data);
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
