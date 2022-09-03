use crate::{ statics::CONFIG };
use reqwest::{Error, Client, Response };
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct Issue {
    url: String,
    number: u32,
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
        .send()
        .await?;

    let issues: Vec<Issue> = 
        response.json()
        .await?;

    Ok(issues)
}

#[tokio::main]
pub async fn create_issue(title: &str, body: &str) -> Result<Issue, Error> {
    let client = Client::new();

    let request_url = format_query_string("issues");

    let request_body = json!({
        "title": title,
        "body": body
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