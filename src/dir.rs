use std::process::Command;

pub fn find_project_filepaths() -> Vec<String> {
    // TODO(#0): make this project agnostic
    let file_type = "*.rs";

    // We leverage `find` because thats a lot easier
    // than trying to reinvent the wheel. We can swallow
    // the cost a shell call
    let output =
        (Command::new("find")
                .arg(".")
                .arg("-name")
                .arg(file_type)
                .output()
                .expect("failed to execute process"))
                .stdout;

    let stringified_output = String::from_utf8(output).unwrap();

    let filepaths: Vec<String> = stringified_output
        .split("\n")
        .filter(|x| x.len() != 0)
        .map(|x| x.to_string())
        .collect();
    
    filepaths
}
