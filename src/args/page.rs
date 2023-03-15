use clap::Args;

use crate::{config::Config, data::ListData};

#[derive(Debug, Args)]
pub struct Arguments {
    /// <INT> : The page number
    pub page: u16,
}

pub fn handle<'a>(config: &Config, command_args: &'a Arguments, list: &'a mut ListData) -> (&'a ListData, Option<u16>) {
    let page_num: u16 = command_args.page;
    if page_num <= 0 {
        println!("Error: page_num out of bounds");
        std::process::exit(exitcode::DATAERR);
    }
    return (list, Some(page_num -1 ));
}