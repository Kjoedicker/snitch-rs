use sqlite::{ Value };

const DATABASE: &str = "snitch-rs.sqlite";

pub fn init() {
    let connection = sqlite::open(DATABASE).unwrap();

    connection
        .execute(
            "
            create table if not exists todos (
                id integer not null unique,
                description text not null,
                todo_line text not null,
                complete integer default 0
            )",
        )
        .unwrap();
}

pub fn insert_todo(id: i64, description: String, todo_line: String, complete: i64) {
    let connection = sqlite::open(DATABASE).unwrap();
    
    let mut cursor = connection
        .prepare("
            insert into todos values (:id, :description, :todo_line, :complete)
        ")
        .unwrap()
        .into_cursor()
        .bind_by_name(vec![
            (":id", Value::Integer(id)), 
            (":description", Value::String(description)),
            (":todo_line", Value::String(todo_line)),
            (":complete", Value::Integer(complete))
        ])
        .unwrap();

    cursor.try_next().unwrap();
}

pub fn delete_todo(id: i64) {
    let connection = sqlite::open(DATABASE).unwrap();
    
    // TODO: do this with a `for X in (...) syntax`
    let mut cursor = connection
        .prepare("
            delete from todos where id = :id
        ")
        .unwrap()
        .into_cursor() // TODO: this shouldn't require a cursor ?
        .bind_by_name(vec![
            (":id", Value::Integer(id)), 
        ])
        .unwrap();

    cursor.try_next().unwrap();
}

pub fn count_todos() -> i64 {
    let connection = sqlite::open(DATABASE).unwrap();

    let mut cursor = connection
        .prepare("select count(id) from todos")
        .unwrap();

    cursor.next().unwrap();

    let count = cursor
        .read::<i64>(0)
        .unwrap();

    count
}
 