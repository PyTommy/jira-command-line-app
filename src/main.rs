
use clap::{Parser, Subcommand};
use serde_derive::{Serialize, Deserialize};
use std::{env, error::Error};
use reqwest::{self, header};
use base64::{encode};
use serde_json::{self};

#[derive(Subcommand)]
enum Commands {
    Issues
}

#[derive(Parser)]
struct Cli {
  #[clap(subcommand)]
  command: Option<Commands>,
}

fn jql_assignee() -> String {
  return String::from("assignee=currentuser()");
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchIssues {
    expand: String,
    start_at: i32,
    max_results: i32,
    total: i32,
    issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Issue {
  expand: String,
  id: String,
  fields: IssueFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IssueFields {
    summary: String,
}

async fn get_issues() -> Result<SearchIssues, Box<dyn Error>> {
  // base url
  let jira_base_url = env::var("JIRA_BASE_URL").unwrap();
  let base = reqwest::Url::parse(&jira_base_url)?;
  
  // authorization
  let jira_auth_text = env::var("JIRA_AUTH").unwrap();
  let auth = encode(String::from(jira_auth_text).as_bytes());
  let authorization = ["Basic", &auth].join(" ");
  
  // create header
  let mut headers = header::HeaderMap::new();
  headers.insert("Authorization", header::HeaderValue::from_str(&authorization)?);

  let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

  let url = base
    .join(&format!("/rest/api/3/search?jql={}", jql_assignee()))?;

  let result: String = client
    .get(url)
    .send()
    .await.expect("failed to request")
    .text()
    .await.expect("failed to deserialize");

  let body: SearchIssues = serde_json::from_str(&result).expect("failed!!");
  Ok(body)
}

#[tokio::main]
async fn main() {
    // let JIRA_API_KEY = env::var("JIRA_API_KEY").is_err();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Issues) => {
            print!("issues");
            let body = get_issues().await.expect("failed get_Issues");
            println!("body = {:?}", serde_json::to_string_pretty(&body).unwrap());
        } 
        None => { 
            println!("no command privided for j")
        }
    }
}