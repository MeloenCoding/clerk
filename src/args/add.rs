use std::vec;

use clap::Args;

use crate::{config::Config, data::{ListData, TaskState, MainTaskFormat, self, Todo}, CommandOutput, create_error, cli::calculate_changed_page};

#[derive(Debug, Args)]
pub struct Arguments {
    /// <STRING>: The task you want to add to the list
    pub task_name: String,

    /// <INT>: Set the command as a subcommand on the given index
    #[arg(short('s'))]
    pub index: Option<usize>,
}

pub fn handle<'a>(config: &Config, command_args: &'a Arguments, list: &'a mut ListData) -> CommandOutput<'a> {
    let index_arg: usize = command_args.index.unwrap_or(usize::MAX);
    let page_num: Option<i64>;

    if command_args.index.is_none() {
        list.push( MainTaskFormat {
            data: vec![],
            state: TaskState::Pending,
            title: command_args.task_name.to_string(),
            github_link: "".to_string(),
        });
        page_num = calculate_changed_page(&list, config, list.len() as i64 - 1);
    }
    else {
        if index_arg == usize::MAX || index_arg > list.len(){
            create_error("invalid index", Some(exitcode::DATAERR));
        }

        list[index_arg].data.append(&mut vec![Todo {
            data: command_args.task_name.to_string(),
            state: TaskState::Pending 
        }]);

        page_num = calculate_changed_page(&list, config, index_arg as i64);
    }

    if config.local {
        data::List::write(list.to_vec(), &config.local_location);
    }
    else if config.remote_location.is_empty() {
        create_error("no remote_location is set", Some(exitcode::CONFIG));
    }
    
    
    return CommandOutput {
        data: list,
        page_num
    };
}