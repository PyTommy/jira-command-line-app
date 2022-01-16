mod jira;

use clap::{Parser, Subcommand};
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

#[tokio::main]
async fn main() {
    // let JIRA_API_KEY = env::var("JIRA_API_KEY").is_err();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Issues) => {
            let result = jira::list().await.expect("failed get_Issues");
            for issue in result.issues {
              println!("[{}] {}", issue.key, issue.fields.summary);
            }
        } 
        None => { 
            println!("no command privided for j")
        }
    }
}