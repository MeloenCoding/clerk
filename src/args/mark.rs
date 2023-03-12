use clap::{Args, Subcommand};

use crate::{config::Config, data::{ListData, MainTaskFormat, TaskState, self}};
#[derive(Debug, Args)]
pub struct Arguments {
    /// The index of the main task you want to mark
    #[arg()]
    pub index_of_maintask: usize,
    /// The index of the sub task you want to mark
    #[arg()]
    pub index_of_subtask: Option<usize>,
    #[clap(subcommand)]
    // pub option: Option<IndexArgs>,
    pub option: IndexArgs
}

#[derive(Debug, Subcommand, )]
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
        println!("Error: index_of_maintask out of bounds");
        std::process::exit(exitcode::DATAERR);
    }
    let mut target_maintask: &mut MainTaskFormat = &mut list[index_of_maintask];

    match index_of_subtask {
        Some(index_of_subtask) => {
            if index_of_subtask >= target_maintask.data.len() {
                println!("Error: index_of_subtask out of bounds");
                std::process::exit(exitcode::CONFIG);
            }
            let mut target_subtask = &mut list[index_of_maintask].data[index_of_subtask];

            target_subtask.state = task_state;
            return list;
        },
        None => {
            target_maintask.state = task_state;
            return list;
        },
    };

}

pub fn handle<'a>(config: Config, command_args: &'a Arguments, list: &'a mut ListData) -> &'a ListData {
    let index_of_maintask: usize = command_args.index_of_maintask;
    let index_of_subtask: Option<usize> = command_args.index_of_subtask;
    
    if index_of_maintask >= list.len() {
        println!("Error: index_of_maintask out of bounds");
        std::process::exit(exitcode::DATAERR);
    }

    if config.local {
        let updated_list: &ListData = match command_args.option {
            IndexArgs::Completed => write_list(TaskState::Completed, index_of_maintask, index_of_subtask, list),
            IndexArgs::Doing => write_list(TaskState::Doing, index_of_maintask, index_of_subtask, list),
            IndexArgs::Pending => write_list(TaskState::Pending, index_of_maintask, index_of_subtask, list),
            IndexArgs::Remove => {
                match index_of_subtask {
                    Some(index_of_subtask) => {
                        let target_maintask: &MainTaskFormat = &list[index_of_maintask];
                        
                        if index_of_subtask >= target_maintask.data.len() {
                            println!("Error: index_of_subtask out of bounds");
                            std::process::exit(exitcode::CONFIG);
                        }
                        list[index_of_maintask].data.remove(index_of_subtask);
                        return list;
                    },
                    None => {
                        list.remove(index_of_maintask);
                        return list;
                    },
                }                
            },
        };
        data::List::write( updated_list.to_vec(), &config.local_location);
    }

    return list;
}