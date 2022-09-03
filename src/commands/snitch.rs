use crate::{ 
    db::{ init, count_issues, insert_issue, delete_issue }, 
    dir::find_project_filepaths, 
    issue::{ ISSUE, structure_issue },
    statics::* 
};
use std::{
    fs::{ write, read_to_string },
    sync::{ Mutex, Arc },
    time::Instant
};
use threadpool::ThreadPool;

fn insert_new_issues( new_issues: Vec<ISSUE>) {
    for issue in new_issues {
        insert_issue(issue.id, issue.description, issue.issue_line, issue.complete);
    }
}

fn delete_completed_issues( completed_issues:Vec<i64>) { 
    for id in completed_issues {
        delete_issue(id);
    }
}

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
    if COMPLETED_ISSUE_PATTERN.is_match(line) {
        pattern = "completed";
    }

    pattern
}

fn process_lines(file: &String, current_issue_count: i64) -> (String, Vec<ISSUE>, Vec<i64>) {
    let mut updated_file_data = String::new();
    let mut new_issues: Vec<ISSUE> = vec![];
    let mut completed_issues: Vec<i64>= vec![];

    let mut issue_counter = current_issue_count;

    for line in file.lines() {
        match match_line(line) {
            "completed" => {
                // TODO: add logic for handling completed lines
                let captured_line = COMPLETED_ISSUE_PATTERN.captures(line).unwrap();

                const ISSUE_NUMBER_INDEX: usize = 1;

                let issue_number = captured_line
                    .get(ISSUE_NUMBER_INDEX)
                    .map(
                        |issue_number| issue_number.as_str().parse::<i64>().unwrap()
                    )
                    .unwrap();

                completed_issues.push(issue_number);
            },
            "untagged" => {
                let structured_issue = 
                    structure_issue(String::from(line), issue_counter);
                
                let current_line = format!(
                    "{}\n", structured_issue.issue_line
                );
    
                new_issues.push(structured_issue);
                
                updated_file_data.push_str(&current_line);
                
                issue_counter += 1;
            },
            _ => {
                updated_file_data.push_str(&format!("{}\n", line))
            }
        }
    }

    (updated_file_data, new_issues, completed_issues)
}

fn process_file (filepath: &String, current_issue_count: i64) -> (String, Vec<ISSUE>, Vec<i64>) {
    let file = read_to_string(&filepath).unwrap();

    let (
        updated_file_data, 
        new_issues, 
        completed_issues
    ) = process_lines(
        &file,
        current_issue_count
    );

    (updated_file_data, new_issues, completed_issues)
}

fn process_files(filepaths: Vec<String>, current_issue_count: i64) {

    let pool = ThreadPool::new(CONFIG.total_threads);
    let issue_counter = Arc::new(Mutex::new(current_issue_count));
    let db_lock = Arc::new(Mutex::new(1));

    for filepath in filepaths {
        let issue_counter = Arc::clone(&issue_counter);
        let db_lock = Arc::clone(&db_lock);

        let thread_file_processing = move || {
            let mut issue_count = issue_counter.lock().unwrap();

            let (updated_file_data, new_issues, completed_issues) = process_file(&filepath, *issue_count);

            let _lock_instance =
                db_lock.lock().unwrap();

            *issue_count += new_issues.len() as i64;

            if !new_issues.is_empty() {
                insert_new_issues(new_issues);
            }
            if !completed_issues.is_empty() {
                delete_completed_issues(completed_issues);
            }

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
    init();
    
    // TODO: should we abstract this out ?
    let now = Instant::now();

    let filepaths = find_project_filepaths();
    let current_issue_count = count_issues();
    println!("{current_issue_count}");

    process_files(filepaths, current_issue_count);

    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
}
