use serde_derive::{Serialize, Deserialize};
use std::{env, error::Error};
use reqwest::{self, header};
use base64::{encode};
use serde_json::{self};
use urlencoding;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchIssuesResult {
    expand: String,
    start_at: i32,
    max_results: i32,
    total: i32,
    pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
  expand: String,
  id: String,
  pub key: String,
  pub fields: IssueFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
    pub summary: String,
    pub assignee: Assignee,
    pub issuetype: IssueType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignee {
    pub display_name: String,
    pub account_id: String,
    pub email_address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
  pub id: String,
  pub description: String,
  pub icon_url: String,
  pub name: String,
  pub subtask: bool,
  pub avatar_id: i32,
  pub hierarchy_level: i32,
}

// =========
// Public
// =========
pub async fn list() -> Result<SearchIssuesResult, Box<dyn Error>> {
  let mut vec_jql = vec![];
  vec_jql.push(jql_assignee_equal_currentuser());
  vec_jql.push(jql_current_sprint());

  let jql = vec_jql.join(" and ");
  let encoded_jql = urlencoding::encode(&jql);
  
  let url = generate_base_url()?
    .join(&format!("/rest/api/3/search?jql={}", encoded_jql))?;

  let result: String = client()?
    .get(url)
    .send()
    .await.expect("failed to request")
    .text()
    .await.expect("failed to deserialize");

  let body: SearchIssuesResult = serde_json::from_str(&result).expect("failed!!");
  Ok(body)
}


// ===========
// Private
// ===========
fn jql_assignee_equal_currentuser() -> String {
  return String::from("assignee=currentuser()");
}
fn jql_current_sprint() -> String {
  return String::from("sprint in openSprints()");
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

pub fn generate_base_url() -> Result<reqwest::Url, Box<dyn Error>> {
  let jira_base_url = env::var("JIRA_BASE_URL").unwrap();
  let base = reqwest::Url::parse(&jira_base_url).unwrap();
  return Ok(base);
}