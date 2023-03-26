use crate::{ 
    helpers::*,
    statics::*
}; 
use reqwest::{Client, StatusCode };
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
        StatusCode::OK => {},
        status_code => panic!("Received error reaching out to github API: {:?}", status_code)
    };

    let issue: Issue = match response.json().await {
        Ok(issue) => issue,
        Err(err) => panic!("Problem marshaling response data into issue type, {:?}", err)
    };

    issue
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    mod fetch_issues {

        use super::*;

        #[tokio::test]
        #[should_panic(expected = "Repo not found")]
        async fn should_handle_404 (){
            let mock_server = MockServer::start().await;

            let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);

            Mock::given(method("GET"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

            let _ = fetch_issues(Some(mock_server.uri())).await;
        }
    
        #[tokio::test]
        #[should_panic(expected = "Request unauthorized, check access token")]
        async fn should_handle_401 (){
            let mock_server = MockServer::start().await;

            let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);

            Mock::given(method("GET"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

            let _ = fetch_issues(Some(mock_server.uri())).await;
        }

        #[tokio::test]
        #[should_panic(expected = "Received error reaching out to github API: 451")]
        async fn should_handle_unexpected_error (){
            let mock_server = MockServer::start().await;

            let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);

            Mock::given(method("GET"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(451))
            .mount(&mock_server)
            .await;

            let _ = fetch_issues(Some(mock_server.uri())).await;
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
    
            let mock_server = MockServer::start().await;

            let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);
        
            Mock::given(method("GET"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
            .mount(&mock_server)
            .await;

            let issues = fetch_issues(Some(mock_server.uri())).await;

            assert_eq!(issues.len(), 2, "There should be a total of two issues");

            assert_eq!(issues[0].html_url, "https://github.com/Kjoedicker/snitch-lab/issues/650");
            assert_eq!(issues[0].number, 650);
            assert_eq!(issues[0].title, " some thing");
        }

        #[tokio::test]
        #[should_panic(expected = "Problem marshaling response data into issue type, reqwest::Error { kind: Decode")]
        async fn should_handle_invalid_response (){

            let json_body = serde_json::json!([{}]);
    
            let mock_server = MockServer::start().await;

            let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);
        
            Mock::given(method("GET"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
            .mount(&mock_server)
            .await;

            let _ = fetch_issues(Some(mock_server.uri())).await;
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
    
            let mock_server = MockServer::start().await;

            let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);
        
            Mock::given(method("POST"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
            .mount(&mock_server)
            .await;

            let issue = create_issue("test-title", Some(mock_server.uri())).await;
        
            assert_eq!(issue.html_url, "https://github.com/Kjoedicker/snitch-lab/issues/650");
            assert_eq!(issue.number, 650);
            assert_eq!(issue.title, " some thing");
        }

        #[tokio::test]
        #[should_panic(expected = "Problem marshaling response data into issue type, reqwest::Error { kind: Decode")]
        async fn should_handle_invalid_response (){

            let json_body = serde_json::json!({});
    
            let mock_server = MockServer::start().await;

            let url_path = format!("/repos/{}/{}/issues", CONFIG.owner, CONFIG.repo);
        
            Mock::given(method("POST"))
            .and(path(url_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_body))
            .mount(&mock_server)
            .await;

            let _ = create_issue("test-title", Some(mock_server.uri())).await;
        }
    }
}
