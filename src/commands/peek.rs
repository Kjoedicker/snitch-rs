use crate::{
    db::{ select_all_issues },
    issue::{ ISSUE }
};
use comfy_table::Table;

fn create_table_from_issues(issues: Vec<ISSUE>) -> Table {
    let mut table = Table::new();

    table.set_header(vec!["Id", "Description", "Complete"]);
        
    for issue in issues {
        table.add_row(
            format!(
                "{}|{}|{}", 
                issue.id, 
                issue.description, 
                (if issue.complete == 1 { true } else { false})
            ).split("|")
        );
    }

    table
}

pub fn peek() {
    let issues = select_all_issues();
    let issue_table = create_table_from_issues(issues);

    println!("{issue_table}");
}
