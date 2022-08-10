
#[macro_use]
extern crate lazy_static;

use regex::{ Regex };
use core::time;
use std::fs::{ write, read_to_string };
use std::sync::{ Mutex, Arc };
use std::thread::sleep;
use std::time::Instant;
use threadpool::ThreadPool;

mod db;
use db::{ count_todos, insert_todo };

mod dir;
use dir::{ aggregrate_files };

mod todo;
use todo::{ TODO, structure_todo };

const TOTAL_THREADS: usize = 10;

lazy_static! {
    static ref UNTAGGED_TODO_PATTERN: Regex = Regex::new(r"^(.*)TODO: (.*)").unwrap();
    static ref COMPLETED_TODO_PATTERN: Regex = Regex::new(r"^(.*)TODO: (.*)").unwrap();
}

fn insert_new_todos( new_todos: Vec<TODO>) {
    for todo in new_todos {
        insert_todo(todo.id, todo.description, todo.todo_line, todo.complete);
    }
}

// fn delete_complete_todos( new_todos: Vec<TODO>) {
//     for todo in new_todos {
//         insert_todo(todo.id, todo.description, todo.todo_line, todo.complete);
//     }
// }

fn update_file(file: &String, file_data: String) {
    write(file, file_data).unwrap_or_else(|err| {
        println!("{file} - Error writing to file: {err}")
    });
}

fn match_line(line: &str) -> &str {
    let mut pattern = "";
    
    if UNTAGGED_TODO_PATTERN.is_match(line) {
        pattern = "untagged";
    }

    // if COMPLETED_TODO_PATTERN.is_match(line) {
    //     pattern = "completed";
    // }

    pattern
}

fn process_lines(file: &String, current_todo_count: i64) -> (String, Vec<TODO>) {
    let mut updated_file_data = String::new();
    let mut new_todos: Vec<TODO> = vec![];
    // let mut completed_todos: Vec<i64>= vec![];

    let mut todo_counter = current_todo_count;

    for line in file.lines() {
        match match_line(line) {
            "completed" => {
                // TODO: add logic for handling completed lines
            },
            "untagged" => {
                let structured_todo = 
                structure_todo(String::from(line), todo_counter);
                
                let current_line = format!(
                    "{}\n", structured_todo.todo_line
                );
    
                new_todos.push(structured_todo);
                
                updated_file_data.push_str(&current_line);
                
                todo_counter += 1;
            },
            _ => {
                updated_file_data.push_str(&format!("{}\n", line))
            }
        }
    }

    (updated_file_data, new_todos)
}

fn process_file (filepath: &String, current_todo_count: i64) -> (String, Vec<TODO>) {
    let file = read_to_string(&filepath).unwrap();

    let (
        updated_file_data, 
        new_todos, 
    ) = process_lines(
        &file,
        current_todo_count
    );

    (updated_file_data, new_todos)
}

fn process_files(filepaths: Vec<String>, current_todo_count: i64) {

    let pool = ThreadPool::new(TOTAL_THREADS);
    let todo_counter = Arc::new(Mutex::new(current_todo_count));
    let db_lock = Arc::new(Mutex::new(1));

    for filepath in filepaths {
        let todo_counter = Arc::clone(&todo_counter);
        let db_lock = Arc::clone(&db_lock);

        let thread_file_processing = move || {
            let mut todo_count = todo_counter.lock().unwrap();

            let (updated_file_data, new_todos) = process_file(&filepath, *todo_count);

            let _lock_instance =
                db_lock.lock().unwrap();

            *todo_count += new_todos.len() as i64;

            insert_new_todos(new_todos);
            // delete_complete_todos(complete_todos);
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

fn main() {
    db::init();
    
    // TODO: should we abstract this out ?
    let now = Instant::now();

    let filepaths = aggregrate_files();
    let current_todo_count = count_todos();
    println!("{current_todo_count}");

    process_files(filepaths, current_todo_count);

    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
}
