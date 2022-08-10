pub struct TODO {
    pub id: i64,
    pub description: String,
    pub todo_line: String,
    pub complete: i64
}

fn split_line_into_strings(line: String) -> (String, String) {
    let lines: Vec<&str> = line.split(':').collect();

    let prefix = String::from(lines[0]);
    let description = String::from(lines[1]);
    
    (prefix, description)
}

pub fn structure_todo(line: String, total_todos: i64) -> TODO {

    let (prefix, description)= 
        split_line_into_strings(line);

    let todo_line = String::from(
        format!("{prefix}(#{total_todos}):{description}")
    );

    let todo = TODO {
        id: total_todos,
        description,
        todo_line: todo_line.clone(),
        complete: 0
    };

    todo
}
