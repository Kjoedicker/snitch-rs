#[derive(Debug)]
pub struct ISSUE {
    pub id: i64,
    pub description: String,
    pub issue_line: String,
    pub complete: i64
}

fn split_line_into_strings(line: String) -> (String, String) {
    let lines: Vec<&str> = line.split(':').collect();

    let prefix = String::from(lines[0]);
    let description = String::from(lines[1]);
    
    (prefix, description)
}

pub fn structure_issue(line: String, total_issues: i64) -> ISSUE {

    let (prefix, description)= 
        split_line_into_strings(line);

    let issue_line = String::from(
        format!("{prefix}(#{total_issues}):{description}")
    );

    let issue = ISSUE {
        id: total_issues,
        description,
        issue_line: issue_line.clone(),
        complete: 0
    };

    issue
}
