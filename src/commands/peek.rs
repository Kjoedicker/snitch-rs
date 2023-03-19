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
                issue.html_url,
                issue.title,
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

mod peek_tests{
    use crate::trackers::github;

    use super::*;

    fn setup() -> Vec<github::Issue> {
        let example_issue_a = Issue {
            number: 1,
            html_url: "exampleurla.nowhere.io".to_string(),
            title: "Example Title A".to_string(),
        };
        let example_issue_b = Issue {
            number: 2,
            html_url: "exampleurlb.nowhere.io".to_string(),
            title: "Example Title B".to_string(),
        };
        let example_issue_c = Issue {
            number: 3,
            html_url: "exampleurlc.nowhere.io".to_string(),
            title: "Example Title C".to_string(),
        };

        let issues = vec![
            example_issue_a.clone(), 
            example_issue_b.clone(), 
            example_issue_c.clone()
        ];

        issues
    }

    #[test]
    fn test_create_table_from_issues() {
        let issues = peek_tests::setup();

        let table = create_table_from_issues(issues.clone());

        let headers = vec!["Id", "Url", "Title"];
    
        for (row_index, row) in table.row_iter().enumerate() {
    
            for (cell_index, cell) in row.cell_iter().enumerate() {
                let content = cell.content();

                match headers[cell_index] {
                    "Id" => {
                        assert_eq!(issues[row_index].number.to_string(), content, "The issue number should match");
                    },
                    "Url" => {
                        assert_eq!(issues[row_index].html_url, content, "The issue URL should match");
                    }, 
                    "Title" => {
                        assert_eq!(issues[row_index].title, content, "The issue title should match");
                    },
                    _ => {
                        assert!(false, "Row content should map to a header");
                    }
                }
            }
        }
    }

    #[test]
    fn test_peek() {
        peek();
    }

}
