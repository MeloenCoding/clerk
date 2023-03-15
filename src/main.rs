mod args;
mod config;
mod data;
mod cli;

use args::{EntityType, add, default, mark, edit, page};
use data::{List, ListData};
use exitcode::{self, ExitCode};
use tokio;
use config::Config;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args: args::ClerkArgs = args::ClerkArgs::parse();
    let config: Config = Config::read();
    let mut list: ListData = List::read(&config).await.data;

    // println!("{:?}", list);

    let output: (&ListData, Option<u16>) = match &args.entity_type {
        Some(EntityType::Add(command_args)) => add::handle(&config, command_args, &mut list),
        Some(EntityType::Mark(command_args)) => mark::handle(&config, command_args, &mut list),
        Some(EntityType::Edit(command_args)) => edit::handle(&config, command_args, &mut list),
        Some(EntityType::Page(command_args)) => page::handle(&config, command_args, &mut list),
        None => default::handle(&list)
    };

    cli::draw_cli(output.0, output.1);

    if !config.local {
        List::set(&config, output.0).await;
    }

    Ok(())
}

pub fn create_error(error_code: ExitCode, custom_error_message: String) {
    println!("Error: {}", custom_error_message);
    std::process::exit(error_code);
}

// clerk 8 -c