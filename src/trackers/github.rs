use crate::{ 
    helpers::*,
    statics::*
}; 
use reqwest::{Error, Client, StatusCode };
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::json;

const GITHUB_URL: &str = "https://api.github.com";

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
    pub number: u32,
}

fn build_request_url(base_url: String) -> String {
    format!("{base_url}/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo)
}

pub async fn fetch_issues(url: Option<String>) -> Vec<Issue> {
    let client = Client::new();
    
    let query_string = build_query_string(vec![
        ("per_page", &CONFIG.issues_per_request)
    ]);

    let request_url = format!("{}?{}", build_request_url(url.unwrap_or(GITHUB_URL.to_string())), query_string);

    let response = client
        .get(request_url)
        .header(USER_AGENT, "SnitchRs")
        .header(AUTHORIZATION, &CONFIG.token)
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => {},
        StatusCode::NOT_FOUND => {
            panic!("Repo not found, check configuration");
        },
        StatusCode::UNAUTHORIZED => {
            panic!("Request unauthorized, check access token");
        }
        status_code => {
            panic!("Received error reaching out to github API: {:?}", status_code);
        }
    }

    let issues: Vec<Issue> = match response.json().await {
        Ok(issues) => issues,
        Err(err) => panic!("Problem marshaling response data into issue type, {:?}", err)
    };
    
    issues
}

#[tokio::main]
pub async fn create_issue(title: &str, url: Option<String>) -> Issue {
    let client = Client::new();

    let request_url = build_request_url(url.unwrap_or(GITHUB_URL.to_string()));

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
        .await
        .unwrap();

    match response.status() {
        StatusCode::CREATED => {},
        status_code => panic!("Received error reaching out to github API: {:?}", status_code)
    };

    let issue: Issue = match response.json().await {
        Ok(issue) => issue,
        Err(err) => panic!("Problem marshaling response data into issue type, {:?}", err)
    };

    issue
}
