use crate::{
    trackers::github::{ Issue, fetch_issues }
};
use comfy_table::Table;

fn create_table_from_issues(issues: Vec<Issue>) -> Table {
    let mut table = Table::new();

    table.set_header(vec!["Id", "Url", "Title"]);
        
    for issue in issues {
        table.add_row(
            format!(
                "{}|{}|{}", 
                issue.number, 
                issue.title,
                issue.html_url
            ).split("|")
        );
    }

    table
}


pub fn peek() {
    let issues = fetch_issues().unwrap();
    let issue_table = create_table_from_issues(issues);
    println!("{issue_table}");
}
