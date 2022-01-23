mod jira;
mod fuzzy;
mod sh;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Issues,
    Commit,
}

#[derive(Parser)]
struct Cli {
  #[clap(subcommand)]
  command: Option<Commands>,

  /// assinee of the task
  #[clap(short, long)]
  assignee: bool,
}



#[tokio::main]
async fn main() {
    // let JIRA_API_KEY = env::var("JIRA_API_KEY").is_err();
    let cli = Cli::parse();
    let select_assignee: bool = cli.assignee;

    match &cli.command {
        Some(Commands::Issues) => {
            let issues = jira::api::list().await.expect("failed get_Issues").issues;
            let issue = jira::utils::select_issue(&issues);
            println!("issue: {}", issue.fields.summary);
        },
        Some(Commands::Commit) => {
            let issues = jira::api::list().await.expect("failed get_Issues").issues;
            let issue = jira::utils::select_issue(&issues);
            let message = format!("{} {}", issue.key, issue.fields.summary);
            sh::git_commit(&message);
        },
        None => todo!(),
    }
}
