mod cli;
mod commands;
mod db;
mod error;
mod models;
mod output;
mod search;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() -> Result<()> {
    let cli = Cli::try_parse();
    
    match cli {
        Ok(cli) => {
            if let Err(e) = run(cli) {
                eprintln!("{}", serde_json::to_string(&serde_json::json!({
                    "error": e.to_string(),
                    "code": e.error_code()
                }))?);
                std::process::exit(1);
            }
        }
        Err(e) => {
            if e.kind() == clap::error::ErrorKind::DisplayHelp {
                println!("{}", e);
            } else if e.kind() == clap::error::ErrorKind::DisplayVersion {
                println!("{}", e);
            } else if std::env::args().len() == 1 {
                output::json::print_schema()?;
            } else {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
    
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    let db = db::init_db()?;
    
    match cli.command {
        Commands::Projects { cmd } => commands::project::handle(cmd, &db),
        Commands::Add(args) => commands::add::handle(args, &db),
        Commands::List(args) => commands::list::handle(args, &db),
        Commands::Search(args) => commands::search::handle(args, &db),
        Commands::Get { id } => commands::get::handle(id, &db),
        Commands::Update(args) => commands::update::handle(args, &db),
        Commands::Delete { id } => commands::delete::handle(id, &db),
    }
}
