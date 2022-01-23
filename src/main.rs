mod jira;
mod fuzzy;
mod sh;

use clap::{Parser, Subcommand};
use std::io::{stdin, stdout, Write};
use colored::*;

#[derive(Subcommand)]
enum SubCommands {
    Issues,
    Commit,
}

#[derive(Parser)]
struct Cli {
  #[clap(subcommand)]
  command: Option<SubCommands>,

  /// assinee of the task
  #[clap(short, long)]
  assignee: bool,
}

fn get_text_input(explanation: &str) -> Option<String> {
    println!("{}", explanation.bright_blue().bold());

    let mut s = String::new();
    let _= stdout().flush();
    match stdin().read_line(&mut s) {
        Ok(_) => return Some(s),
        Err(_) => None,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let select_assignee: bool = cli.assignee;

    match &cli.command {
        Some(SubCommands::Issues) => {
            let issues = jira::api::list().await.expect("failed get_Issues").issues;
            let issue = jira::utils::select_issue(&issues);
            println!("issue: {}", issue.fields.summary);
        },
        Some(SubCommands::Commit) => {
            let issues = jira::api::list().await.expect("failed get_Issues").issues;
            let issue = jira::utils::select_issue(&issues);

            let default_commit_message = &issue.fields.summary;
            let explanation = format!("Enter commit message. (default={})", default_commit_message.yellow());
            let user_input = get_text_input(&explanation);
            
            match user_input {
                Some(input) => sh::git_commit(&format!("{} {}", issue.key, &input)),
                None => sh::git_commit(&format!("{} {}", issue.key, default_commit_message)),
            }
        },
        None => {
            println!("enter `j -h` for help");
        },
    }
}
