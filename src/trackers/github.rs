use crate::{ 
    helpers::*,
    statics::*
}; 
use reqwest::{Error, Client, StatusCode };
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
    pub number: u32,
}

fn build_request_url() -> String {
    const BASE_URL: &str = "https://api.github.com/repos";

    return format!("{BASE_URL}/{}/{}/issues", CONFIG.owner, CONFIG.repo);
}

#[tokio::main]
pub async fn fetch_issues() -> Result<Vec<Issue>, Error> {
    let client = Client::new();

    let query_string = build_query_string(vec![
        ("per_page", &CONFIG.issues_per_request)
    ]);
    
    let request_url = format!("{}?{}", build_request_url(), query_string);

    let response = client
        .get(request_url)
        .header(USER_AGENT, "SnitchRs")
        .header(AUTHORIZATION, &CONFIG.token)
        .send()
        .await?;

    match response.status() {
        StatusCode::NOT_FOUND => {
            panic!("Repo was not found");
        },
        StatusCode::UNAUTHORIZED => {
            panic!("Request unauthorized, check access token");
        }
        _ => {
            println!("fetch_issues(): Received error reaching to github API: {:?}", response.status());
        }
    }

    let issues: Vec<Issue> = 
        response.json()
        .await?;

    Ok(issues)
}

#[tokio::main]
pub async fn create_issue(title: &str) -> Result<Issue, Error> {
    let client = Client::new();

    let request_url = build_request_url();

    let request_body = json!({
        "title": title,
        "body": ""
    });

    let response = client
        .post(request_url)
        .header(USER_AGENT, "SnitchRs")
        .header(AUTHORIZATION, &CONFIG.token)
        .json(&request_body)
        .send()
        .await?;

    let issue: Issue = 
        response.json()
        .await?;

    Ok(issue)
}