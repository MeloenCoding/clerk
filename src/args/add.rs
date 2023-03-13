use std::vec;

use clap::Args;

use crate::{config::Config, data::{ListData, TaskState, MainTaskFormat, self, Todo}};

#[derive(Debug, Args)]
pub struct Arguments {
    /// <STRING>: The task you want to add to the list
    pub task_name: String,

    /// <INT>: Set the command as a subcommand on the given index
    #[arg(short('s'))]
    pub index: Option<usize>,
}

pub fn handle<'a>(config: Config, command_args: &'a Arguments, list: &'a mut ListData) -> (&'a ListData, Option<u16>) {
    let index_arg: usize = command_args.index.unwrap_or(usize::MAX);
    if config.local {
        if command_args.index.is_none() {
            list.push( MainTaskFormat {
                data: vec![],
                state: TaskState::Pending,
                title: command_args.task_name.to_string(),
                github_link: "".to_string(),
            });
        }
        else {
            if index_arg == usize::MAX || index_arg > list.len(){
                println!("Error: invalid index");
                std::process::exit(exitcode::DATAERR);
            }

            list[index_arg].data.append(&mut vec![Todo {
                data: command_args.task_name.to_string(),
                state: TaskState::Pending 
            }]);
        }
    }
    else if config.remote_location.is_empty() {
        println!("Error: no remote_location is set");
        std::process::exit(exitcode::CONFIG);
    }
    else{
        todo!();
    }

    data::List::write(list.to_vec(), &config.local_location);
    return (list, None);
}