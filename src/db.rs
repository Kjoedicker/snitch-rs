use crate::{statics::CONFIG, issue::ISSUE};
use sqlite::{ Value, State, Statement };

pub fn init() {
    // TODO: find a way to break this redundant connection pattern
    let connection = sqlite::open(CONFIG.database_name.as_str()).unwrap();

    connection
        .execute(
            "
            create table if not exists issues (
                id integer not null unique,
                description text not null,
                issue_line text not null,
                complete integer default 0
            )",
        )
        .unwrap();
}

pub fn insert_issue(id: i64, description: String, issue_line: String, complete: i64) {
    let connection = sqlite::open(CONFIG.database_name.as_str()).unwrap();
    
    let mut cursor = connection
        .prepare("
            insert into issues values (:id, :description, :issue_line, :complete)
        ")
        .unwrap()
        .into_cursor()
        .bind_by_name(vec![
            (":id", Value::Integer(id)), 
            (":description", Value::String(description)),
            (":issue_line", Value::String(issue_line)),
            (":complete", Value::Integer(complete))
        ])
        .unwrap();

    cursor.try_next().unwrap();
}

pub fn delete_issue(id: i64) {
    let connection = sqlite::open(CONFIG.database_name.as_str()).unwrap();
    
    // TODO: do this with a `for X in (...) syntax`
    let mut cursor = connection
        .prepare("
            delete from issues where id = :id
        ")
        .unwrap()
        .into_cursor() // TODO: this shouldn't require a cursor ?
        .bind_by_name(vec![
            (":id", Value::Integer(id)), 
        ])
        .unwrap();

    cursor.try_next().unwrap();
}

pub fn count_issues() -> i64 {
    let connection = sqlite::open(CONFIG.database_name.as_str()).unwrap();

    let mut cursor = connection
        .prepare("select count(id) from issues")
        .unwrap();

    cursor.next().unwrap();

    let count = cursor
        .read::<i64>(0)
        .unwrap();

    count
}

fn map_rows_to_issues(mut cursor: Statement) -> Vec<ISSUE> {
    let mut issues: Vec<ISSUE> = vec![];

    while let State::Row = cursor.next().unwrap() {
        let id = cursor.read::<i64>(0).unwrap();
        let description = cursor.read::<String>(1).unwrap();
        let issue_line = cursor.read::<String>(2).unwrap();
        let complete = cursor.read::<i64>(3).unwrap();

        let issue = ISSUE { id, description, issue_line, complete };

        issues.push(issue);
    }

    issues
}

pub fn select_all_issues() -> Vec<ISSUE> {
    let connection = sqlite::open(CONFIG.database_name.as_str()).unwrap();

    let cursor = connection
        .prepare("
            select * from issues
        ")
        .unwrap();

    map_rows_to_issues(cursor)
}
