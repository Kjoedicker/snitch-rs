use crate::{
    db::{ select_all_todos },
    todo::{ TODO }
};
use comfy_table::Table;

fn create_table_from_todos(todos: Vec<TODO>) -> Table {
    let mut table = Table::new();

    table.set_header(vec!["Id", "Description", "Complete"]);
        
    for todo in todos {
        table.add_row(
            format!(
                "{}|{}|{}", 
                todo.id, 
                todo.description, 
                (if todo.complete == 1 { true } else { false})
            ).split("|")
        );
    }

    table
}

pub fn peek() {
    let todos = select_all_todos();
    let todo_table = create_table_from_todos(todos);

    println!("{todo_table}");
}
