use clap::{Args, Subcommand};

use crate::{config::Config, data::{ListData, MainTaskFormat, Todo, self}};

#[derive(Debug, Args)]
pub struct Arguments {
    /// <INT> : The index of the main task you want to mark
    pub index_of_maintask: usize,
    /// <STRING> : The value of the property that you would like to change it to
    pub new_value: String,
    /// <INT> : The index of the sub task you want to mark
    pub index_of_subtask: Option<usize>,

    #[clap(subcommand)]
    pub option: IndexArgs
}

#[derive(Debug, Subcommand)]
pub enum IndexArgs {
    /// Mark todo as completed
    #[clap(short_flag('t'))]
    Title,
    /// Remove todo from the List
    #[clap(short_flag('l'))]
    Link
}

fn write_list<'a>(new_value: &'a String, index_of_maintask: usize, index_of_subtask: Option<usize>, list: &'a mut ListData) -> &'a ListData {
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
            let mut target_subtask: &mut Todo = &mut list[index_of_maintask].data[index_of_subtask];

            target_subtask.data = new_value.to_owned();
            return list;
        },
        None => {
            target_maintask.title = new_value.to_owned();
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
            IndexArgs::Title => write_list(&command_args.new_value, index_of_maintask, index_of_subtask, list),
            IndexArgs::Link => todo!(),
        };
        data::List::write(updated_list.to_vec(), &config.local_location);
    }

    return list;
}
