use std::{
    process::{ Command },
};

pub fn format_issues(issues: Vec<String>) -> String {

    let concated_issues = format!(
        "#{}", 
        issues.join(", #")
    );

    concated_issues
}

pub fn format_commit_message(issues: &String) -> String {

    let base_message = format!(
        "Adding {}", 
        match issues.len() > 1 {
            true => "issues: ",
            _ => "issue: "
        }
    );

    let commit_message = format!(
        "{}{}",
        base_message, 
        issues
    );

    commit_message
}

fn commit_file(filepath: &str, commit_message: String) {

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .arg("--include")
        .arg(filepath)
        .output()
        .expect(
            &format!(
                "Failed to commit '{}'\n for: {}`", 
                commit_message,
                filepath 
            )
        ).stdout;
}

pub fn commit_reported_issues(filepath: &str, issues: Vec<String>) {

    let formatted_issues = format_issues(issues);
    let commit_message= format_commit_message(&formatted_issues);

    commit_file(&filepath, commit_message);

    println!("[COMMITTED] issues: {}", formatted_issues);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_string(val: &str) -> String {
        String::from(val)
    }

    fn setup () -> Vec<String> {
        let test_issues: Vec<String> = vec![
            str_to_string("1"),
            str_to_string("2"),
            str_to_string("3")
        ];

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
        fn formats_a_commit_message() {
            let test_issues = setup();

            let formatted_issues = format_issues(test_issues);

            let commit_message = format_commit_message(&formatted_issues);

            let expectation = true;
            let reality = commit_message == "Adding issues: #1, #2, #3";

            assert_eq!(expectation, reality);
        }
    }
}