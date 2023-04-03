mod args;
mod config;
mod data;
mod cli;

use args::{EntityType, add, default, mark, edit, page};
use colored::Colorize;
use data::{List, ListData};
use exitcode::{self, ExitCode};
use tokio;
use config::Config;
use clap::Parser;

pub struct CommandOutput<'a> {
    pub data: &'a ListData,
    pub page_num: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args: args::ClerkArgs = args::ClerkArgs::parse();
    let config: Config = Config::read();
    let mut list: ListData = List::read(&config).await.data;

    // println!("{:?}", list);

    let output: CommandOutput = match &args.entity_type {
        Some(EntityType::Add(command_args)) => add::handle(&config, command_args, &mut list),
        Some(EntityType::Mark(command_args)) => mark::handle(&config, command_args, &mut list),
        Some(EntityType::Edit(command_args)) => edit::handle(&config, command_args, &mut list),
        Some(EntityType::Page(command_args)) => page::handle(&config, command_args, &mut list),
        None => default::handle(&list)
    };

    cli::draw_cli(&output, &config);

    if !config.local {
        List::set(&config, output.data).await;
    }

    Ok(())
}

pub fn create_error(custom_error_message: &str, error_code: Option<ExitCode>) {
    println!("clerk v{} \nError: {}", env!("CARGO_PKG_VERSION"), custom_error_message,);
    println!("If you think this is a bug or you don't know how to resolve it, open an issue on my github here: {}", env!("CARGO_PKG_REPOSITORY").bold());
    if error_code.is_some() {
        println!("Errorcode {}", error_code.unwrap());
        std::process::exit(error_code.unwrap());
    };
}
