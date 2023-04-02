use super::tracker::{IssueTracker, Issue};
use crate::config::Config;
use crate::{ 
    helpers::*
}; 
use async_trait::async_trait;
use reqwest::{Client, StatusCode };
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use serde_json::json;

pub struct Github {
    base_tracker_url: String,
    owner: String,
    repo: String,
    issues_per_request: String,
    token: String
}

pub fn init_instance(config: Config) -> Github {
    Github {
        base_tracker_url: config.base_tracker_url,
        owner: config.owner,
        issues_per_request: config.issues_per_request,
        repo: config.repo,
        token: config.token
    }
}

#[async_trait]
impl IssueTracker for Github {
    fn build_request_url(&self) -> String {
        format!("{}/repos/{}/{}/issues", self.base_tracker_url, self.owner, self.repo)
    }

    async fn fetch_issues(&self) -> Vec<Issue> {
        let client = Client::new();
        
        let query_string = build_query_string(vec![
            ("per_page", &self.issues_per_request)
        ]);

        let request_url = format!("{}?{}", self.build_request_url(), query_string);

        let response = client
            .get(request_url)
            .header(USER_AGENT, "SnitchRs")
            .header(AUTHORIZATION, &self.token)
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

    async fn create_issue(&self, title: &str) -> Issue {
        let client = Client::new();

        let request_url = self.build_request_url();

        let request_body = json!({
            "title": title,
            "body": ""
        });

        let response = client
            .post(request_url)
            .header(USER_AGENT, "SnitchRs")
            .header(AUTHORIZATION, &self.token)
            .json(&request_body)
            .send()
            .await
            .unwrap();

        match response.status() {
            StatusCode::CREATED => {},
            StatusCode::OK => {},
            status_code => panic!("Received error reaching out to github API: {:?}", status_code)
        };

        let issue: Issue = match response.json().await {
            Ok(issue) => issue,
            Err(err) => panic!("Problem marshaling response data into issue type, {:?}", err)
        };

        issue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::init;
    use lazy_static::lazy_static;
    use serde_json::Value;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    lazy_static!{
        #[derive(Debug)]
        pub static ref CONFIG: Config = init();
    }

    pub fn build_instance(base_tracker_url: String) -> Github {
        Github { 
            base_tracker_url: base_tracker_url, 

            // default
            repo: CONFIG.repo.clone(), 
            owner: CONFIG.owner.clone(), 
            issues_per_request: CONFIG.issues_per_request.clone(),
            token: CONFIG.token.clone()
        }
    }

    pub async fn build_mock_server(method_type: &str, status: StatusCode, json_body: Option<Value>) -> (MockServer, String) {
        let mock_server = MockServer::start().await;

        let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);

        let response = match json_body {
            Some(data) => {
                ResponseTemplate::new(status).set_body_json(data)
            },
            None => {
                ResponseTemplate::new(status)
            }
        };

        Mock::given(method(method_type))
        .and(path(url_path.clone()))
        .respond_with(response)
        .mount(&mock_server)
        .await;

        let server_uri = mock_server.uri();
    
        (mock_server, server_uri)
    }

    mod fetch_issues {
        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Repo not found")]
        async fn should_handle_404 (){
            let (
                _mock_server, 
                server_uri
            ) = build_mock_server("GET", StatusCode::NOT_FOUND, None).await;

            let github_tracker = build_instance(server_uri);

            let _ = github_tracker.fetch_issues().await;
        }
    
        #[tokio::test]
        #[should_panic(expected = "Request unauthorized, check access token")]
        async fn should_handle_401 (){
            let (
                _mock_server, 
                server_uri
            ) = build_mock_server("GET", StatusCode::UNAUTHORIZED, None).await;

            let github_tracker = build_instance(server_uri);

            let _ = github_tracker.fetch_issues().await;
        }

        #[tokio::test]
        #[should_panic(expected = "Received error reaching out to github API: 451")]
        async fn should_handle_unexpected_error (){
            let (
                _mock_server, 
                server_uri
            ) = build_mock_server("GET", StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, None).await;

            let github_tracker = build_instance(server_uri);

            let _ = github_tracker.fetch_issues().await;
        }

        #[tokio::test]
        async fn should_handle_valid_response (){

            let json_body = serde_json::json!([{
                "html_url": "https://github.com/Kjoedicker/snitch-lab/issues/650",
                "number": 650,
                "title": " some thing",
              },
              {
                "html_url": "https://github.com/Kjoedicker/snitch-lab/issues/650",
                "number": 650,
                "title": " some thing",
              },
            ]);
    
            let (
                _mock_server, 
                server_uri
            ) = build_mock_server("GET", StatusCode::OK, Some(json_body)).await;

            let github_tracker = build_instance(server_uri);

            let issues = github_tracker.fetch_issues().await;

            assert_eq!(issues.len(), 2, "There should be a total of two issues");

            assert_eq!(issues[0].html_url, "https://github.com/Kjoedicker/snitch-lab/issues/650");
            assert_eq!(issues[0].number, 650);
            assert_eq!(issues[0].title, " some thing");
        }

        #[tokio::test]
        #[should_panic(expected = "Problem marshaling response data into issue type, reqwest::Error { kind: Decode")]
        async fn should_handle_invalid_response (){

            let json_body = serde_json::json!([{}]);
    
            let (
                _mock_server, 
                server_uri
            ) = build_mock_server("GET", StatusCode::OK, Some(json_body)).await;

            let github_tracker = build_instance(server_uri);

            let _ = github_tracker.fetch_issues().await;
        }
    }

    mod create_issue {
        use super::*;

        #[tokio::test]
        async fn should_handle_valid_response (){

            let json_body = serde_json::json!({
                "html_url": "https://github.com/Kjoedicker/snitch-lab/issues/650",
                "number": 650,
                "title": " some thing",
              }
            );
    
            let (
                _mock_server, 
                server_uri
            ) = build_mock_server("POST", StatusCode::OK, Some(json_body)).await;

            let github_tracker = build_instance(server_uri);

            let issue = github_tracker.create_issue("test-title").await;
        
            assert_eq!(issue.html_url, "https://github.com/Kjoedicker/snitch-lab/issues/650");
            assert_eq!(issue.number, 650);
            assert_eq!(issue.title, " some thing");
        }

        #[tokio::test]
        #[should_panic(expected = "Problem marshaling response data into issue type, reqwest::Error { kind: Decode")]
        async fn should_handle_invalid_response (){

            let json_body = serde_json::json!({});
    
            let (
                _mock_server, 
                server_uri
            ) = build_mock_server("POST", StatusCode::OK, Some(json_body)).await;

            let github_tracker = build_instance(server_uri);

            let _ = github_tracker.create_issue("test-title").await;
        }
    }
}
