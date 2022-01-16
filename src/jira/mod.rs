use serde_derive::{Serialize, Deserialize};
use std::{env, error::Error};
use reqwest::{self, header};
use base64::{encode};
use serde_json::{self};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchIssuesResult {
    expand: String,
    start_at: i32,
    max_results: i32,
    total: i32,
    issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
  expand: String,
  id: String,
  fields: IssueFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
    summary: String,
}

pub async fn list() -> Result<SearchIssuesResult, Box<dyn Error>> {

  let url = generate_base_url()?
    .join(&format!("/rest/api/3/search?jql={}", jql_assignee_equal_currentuser()))?;

  let result: String = client()?
    .get(url)
    .send()
    .await.expect("failed to request")
    .text()
    .await.expect("failed to deserialize");

  let body: SearchIssuesResult = serde_json::from_str(&result).expect("failed!!");
  Ok(body)
}

fn jql_assignee_equal_currentuser() -> String {
  return String::from("assignee=currentuser()");
}

fn client() -> Result<reqwest::Client, Box<dyn Error>> {  
  let basic_auth = generate_basic_auth()?;
  
  let mut headers = header::HeaderMap::new();
  headers.insert("Authorization", header::HeaderValue::from_str(&basic_auth)?);

  let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;
  
  return Ok(client);
}

fn generate_basic_auth() -> Result<String, Box<dyn Error>> {
  let jira_auth_text = env::var("JIRA_AUTH").unwrap();
  let auth = encode(String::from(jira_auth_text).as_bytes());
  let authorization = ["Basic", &auth].join(" ");
  return Ok(authorization);
}

fn generate_base_url() -> Result<reqwest::Url, Box<dyn Error>> {
  let jira_base_url = env::var("JIRA_BASE_URL").unwrap();
  let base = reqwest::Url::parse(&jira_base_url).unwrap();
  return Ok(base);
}