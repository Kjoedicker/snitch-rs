use crate::{ statics::CONFIG };
use reqwest::{Error, Client, Response, StatusCode };
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
    pub number: u32,
}

fn format_query_string(postfix: &str) -> String {
    const BASE_URL: &str = "https://api.github.com/repos";

    let request_url = format!("{BASE_URL}/{}/{}/{}", CONFIG.owner, CONFIG.repo, postfix);

    request_url
}

#[tokio::main]
pub async fn fetch_issues() -> Result<Vec<Issue>, Error> {
    let client = Client::new();

    let request_url = format_query_string("issues?per_page=100");

    // TODO: handle 404 situation, or anything that might be less than expected
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
            // Uncomment for debug
            println!("fetch_issues(): Recieved error reaching to github API: {:?}", response.status());
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

    let request_url = format_query_string("issues");

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