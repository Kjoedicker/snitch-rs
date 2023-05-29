use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
    pub number: u32,
    pub state: String,
}

#[async_trait]
pub trait IssueTracker {
    fn build_request_url(&self) -> String;
    async fn fetch_issues(&self) -> Vec<Issue>;
    async fn fetch_issue(&self, issue_number: &str) -> Issue;
    async fn create_issue(&self, title: &str) -> Issue;
}
