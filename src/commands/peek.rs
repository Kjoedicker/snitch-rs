use crate::{
    config::{init},
    trackers::{
        tracker::{Issue, IssueTracker}, 
        github::{init_instance}
    }
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
            ).split('|')
        );
    }

    table
}

pub async fn peek() {
    let config = init();
    let issue_tracker = init_instance(config);

    let issues = issue_tracker.fetch_issues().await;
    let issue_table = create_table_from_issues(issues);

    println!("{issue_table}");
}

#[cfg(test)]
mod tests{
    use super::*;

    fn setup() -> Vec<Issue> {
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

    mod create_table_from_issues {
        use super::*;

        #[test]
        fn maps_each_issue_field_to_the_expected_header() {
            let issues = setup();

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
    }

    mod peek {
        use super::*;

        #[tokio::test]
        async fn should_successfully_run() {
            peek().await;
        }
    }

}
