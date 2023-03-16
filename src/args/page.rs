use clap::Args;

use crate::{config::Config, data::ListData, CommandOutput, create_error};

#[derive(Debug, Args)]
pub struct Arguments {
    /// <INT> : The page number
    pub page: i64,
}

pub fn handle<'a>(_config: &Config, command_args: &'a Arguments, list: &'a mut ListData) -> CommandOutput<'a> {
    let page_num: i64 = command_args.page;
    if page_num <= 0 {
        create_error("page_num out of bounds", Some(exitcode::DATAERR));
    }
    CommandOutput {
        data: list,
        page_num: Some(page_num - 1),
    }
}