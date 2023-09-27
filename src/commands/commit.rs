use core::panic;
use lazy_static::lazy_static;
use std::process::Command;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref COMMIT_ACTION: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}

pub fn format_issues(issues: Vec<String>) -> String {
    let concated_issues = format!("#{}", issues.join(", #"));

    concated_issues
}

pub fn format_commit_message(commit_type: &str, issues: &String) -> String {
    let base_message = format!(
        "{} {}",
        commit_type,
        match issues.is_empty() {
            true => "issue: ",
            _ => "issues: ",
        }
    );

    let commit_message = format!("{}{}", base_message, issues);

    commit_message
}

fn stage_file(filepath: &str) {
    let result = Command::new("git").arg("add").arg(filepath).output();

    let output = match result {
        Ok(option) => option,
        Err(err) => {
            panic!("Error staging file {filepath}\nContext: {:?}", err)
        }
    };

    match output.status.success() {
        true => {}
        false => {
            panic!("Error staging file {filepath}\nContext: {:?}", output)
        }
    }
}

fn commit_staged(filepath: &str, commit_message: String) {
    let result = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .output();

    let output = match result {
        Ok(option) => option,
        Err(err) => {
            panic!("Error committing file {filepath}\nContext: {:?}", err)
        }
    };

    match output.status.success() {
        true => {}
        false => {
            panic!("Error committing file {filepath}\nContext: {:?}", output)
        }
    }
}

pub fn commit_issues(commit_type: &str, filepath: &str, issues: Vec<String>) {
    // This stops a race condition when `commit_reported_issues`
    // is called at the same time across threads
    let power_to_commit = Arc::clone(&COMMIT_ACTION);
    let _lock_power_to_commit = power_to_commit.lock().unwrap();

    let formatted_issues = format_issues(issues);
    let commit_message = format_commit_message(commit_type, &formatted_issues);

    stage_file(filepath);
    commit_staged(filepath, commit_message);

    println!(
        "[{}] issues: {}",
        commit_type.to_ascii_uppercase(),
        formatted_issues
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_string(val: &str) -> String {
        String::from(val)
    }

    fn setup() -> Vec<String> {
        let test_issues: Vec<String> =
            vec![str_to_string("1"), str_to_string("2"), str_to_string("3")];

        test_issues
    }

    mod format_issues {
        use super::*;

        #[test]
        fn formats_issues() {
            let test_issues = setup();

            let formatted_issues = format_issues(test_issues);

            let expectation = true;
            let reality = formatted_issues == "#1, #2, #3";

            assert_eq!(expectation, reality, "Issues should be formatted properly");
        }
    }

    mod format_commit_message {
        use super::*;

        #[test]
        fn formats_a_adding_commit_message() {
            let test_issues = setup();

            let formatted_issues = format_issues(test_issues);

            let commit_message = format_commit_message("Adding", &formatted_issues);

            let expectation = true;
            let reality = commit_message == "Adding issues: #1, #2, #3";

            assert_eq!(expectation, reality);
        }
        #[test]

        fn formats_a_removing_commit_message() {
            let test_issues = setup();

            let formatted_issues = format_issues(test_issues);

            let commit_message = format_commit_message("Removing", &formatted_issues);

            let expectation = true;
            let reality = commit_message == "Removing issues: #1, #2, #3";

            assert_eq!(expectation, reality);
        }
    }
}
