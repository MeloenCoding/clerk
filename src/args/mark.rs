use clap::{Args, Subcommand};

use crate::{config::Config, data::{ListData, MainTaskFormat, TaskState, self, Todo}, CommandOutput, create_error};
#[derive(Debug, Args)]
pub struct Arguments {
    /// <INT> : The index of the main task you want to mark
    pub index_of_maintask: usize,
    /// <INT> : The index of the sub task you want to mark
    pub index_of_subtask: Option<usize>,

    #[clap(subcommand)]
    pub option: IndexArgs
}

#[derive(Debug, Subcommand)]
pub enum IndexArgs {
    /// Mark todo as completed
    #[clap(short_flag('c'))]
    Completed,
    /// Mark todo as doing
    #[clap(short_flag('d'))]
    Doing,
    /// Mark todo as pending
    #[clap(short_flag('p'))]
    Pending,
    /// Remove todo from the List
    #[clap(short_flag('r'))]
    Remove
}

fn write_list(task_state: TaskState, index_of_maintask: usize, index_of_subtask: Option<usize>, list: &mut ListData) -> &ListData {
    if index_of_maintask >= list.len() {
        create_error("index_of_maintask out of bounds", Some(exitcode::DATAERR));
    }
    let mut target_maintask: &mut MainTaskFormat = &mut list[index_of_maintask];

    match index_of_subtask {
        Some(index_of_subtask) => {
            if index_of_subtask >= target_maintask.data.len() {
                create_error("index_of_subtask out of bounds", Some(exitcode::CONFIG));
            }
            let mut target_subtask: &mut Todo = &mut list[index_of_maintask].data[index_of_subtask];

            target_subtask.state = task_state;
            return list;
        },
        None => {
            target_maintask.state = task_state;
            return list;
        },
    };
}

pub fn handle<'a>(config: &Config, command_args: &'a Arguments, list: &'a mut ListData) -> CommandOutput<'a> {
    let index_of_maintask: usize = command_args.index_of_maintask;
    let index_of_subtask: Option<usize> = command_args.index_of_subtask;
    
    if index_of_maintask >= list.len() {
        create_error("index_of_maintask out of bounds", Some(exitcode::DATAERR));
    }

    let updated_list: &ListData = match command_args.option {
        IndexArgs::Completed => write_list(TaskState::Completed, index_of_maintask, index_of_subtask, list),
        IndexArgs::Doing => write_list(TaskState::Doing, index_of_maintask, index_of_subtask, list),
        IndexArgs::Pending => write_list(TaskState::Pending, index_of_maintask, index_of_subtask, list),
        IndexArgs::Remove => {
            match index_of_subtask {
                Some(index_of_subtask) => {
                    let target_maintask: &MainTaskFormat = &list[index_of_maintask];
                    
                    if index_of_subtask >= target_maintask.data.len() {
                        create_error("index_of_subtask out of bounds", Some(exitcode::CONFIG));
                    }
                    list[index_of_maintask].data.remove(index_of_subtask);
                    return CommandOutput {
                        data: list,
                        page_num: None
                    };
                },
                None => {
                    list.remove(index_of_maintask);
                    return CommandOutput {
                        data: list,
                        page_num: None
                    };
                },
            }                
        },
    };
    if config.local {
        data::List::write( updated_list.to_vec(), &config.local_location);
    }

    return CommandOutput {
        data: list,
        page_num: None
    };
}