use std::process::Command;

pub fn find_project_filepaths() -> Vec<String> {
    // TODO: make the `file_type` more language agnostic and configurable
    let file_type = "*.rs";

    // TODO: make the exclusion list more configurable
    let exclusion = "./src/statics.rs";

    // We leverage `find` because thats a lot easier
    // than trying to reinvent the wheel. 
    // We can swallow the cost of a shell call
    let output =
        (Command::new("find")
                .arg(".")
                .arg("-name")
                .arg(file_type)
                .arg("-and")
                .arg("-not")
                .arg("-path")
                .arg(exclusion)
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

mod tests {
    mod find_project_filepaths {
        use regex::Regex;
        use crate::dir::find_project_filepaths;

        #[test]
        fn finds_files () {
            let filepaths = find_project_filepaths();
            
            // TODO: when filetypes and exclusions are fine tuned make this testing more thorough. If possible
            let minimum_number_of_rust_files = 10;
            
            assert_eq!(filepaths.len() >= minimum_number_of_rust_files, true, "find_project_files should return a list of files");
        }

        #[test]
        fn properly_includes () {
            let filepaths = find_project_filepaths();

            let expected_file_type = Regex::new("^.*.(rs|RS)$").unwrap();
            
            filepaths.into_iter().for_each(|filepath| {
               assert!(expected_file_type.is_match(&filepath), "Should have matched the expected file type");  
            });
        }

        #[test]
        fn properly_excludes () {
            let filepaths = find_project_filepaths();

            let exclusion = "./src/statics.rs";    
            
            filepaths.into_iter().for_each(|filepath| {
               assert_ne!(filepath, exclusion, "Should have excluded {exclusion}");
            });
        }
    }
}