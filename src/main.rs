mod jira;

use clap::{Parser, Subcommand};
use skim::prelude::*;
use std::{io::Cursor, ops::Index};


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
async fn main()  {
    // let JIRA_API_KEY = env::var("JIRA_API_KEY").is_err();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Issues) => {
            let result = jira::list().await.expect("failed get_Issues");

            let vec_items = result.issues
                .into_iter()
                .map(|issue| format!("[{}] {}", issue.key, issue.fields.summary))
                .collect::<Vec<String>>();
            
            // https://github.com/lotabout/skim#use-as-a-library
            let options = SkimOptionsBuilder::default()
                .build()
                .unwrap();
            let input = vec_items.join("\n").to_string();
            let item_reader = SkimItemReader::default();
            let items = item_reader.of_bufread(Cursor::new(input));
            let selected_items = Skim::run_with(&options, Some(items))
              .map(|out| out.selected_items)
              .unwrap_or_else(|| Vec::new());

            for item in selected_items.iter() {
                let string = format!("{}", item.output());
                println!("{}", string);
                let index = vec_items.iter().position(|el| el == &string);
                match index {
                    Some(i) => {
                        println!("index: {}", i);
                    },
                    None => println!("index: none"), 
                }
            }
        } 
        None => { 
            println!("no command privided for j")
        }
    }
}