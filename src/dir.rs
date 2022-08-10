// MOCK
pub fn aggregrate_files() -> Vec<String>{
    // TODO: align this with reality
    let mut filepaths = vec![];

    for i in 1..51 {
        filepaths.push(format!("./test-todos/todo{i}.rs"));
    }

    filepaths
}