use clap::{Args, Subcommand};

use crate::{config::Config, data::{ListData, MainTaskFormat, Todo}};
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


pub fn handle<'a>(config: Config, command_args: &'a Arguments, list: &'a ListData) -> &'a ListData {
    let index_of_maintask: usize = command_args.index_of_maintask;
    
    if index_of_maintask >= list.len() {
        println!("Error: index_of_maintask out of bounds");
        std::process::exit(exitcode::DATAERR);
    }

    match command_args.index_of_subtask {
        Some(index_of_subtask) => {
            if index_of_subtask >= target_task.data.len() {
                println!("Error: index_of_subtask out of bounds");
                std::process::exit(exitcode::CONFIG);
            }

            let target_task: &Todo = &list[index_of_maintask].data[index_of_subtask];

        },
        None => {

        },
    }

    

    println!("{:?} {:?}", target_task, target_subtask);

    if config.local {
        let mut updated_task: MainTaskFormat = match command_args.option {
            IndexArgs::Completed => {
                todo!()
            },
            IndexArgs::Doing => todo!(),
            IndexArgs::Pending => todo!(),
            IndexArgs::Remove => todo!(),
        };

    //     if command_args.completed || command_args.doing || command_args.remove {
            
    //     }
    //     else {
    //         if index_arg == usize::MAX || index_arg > list.len(){
    //             println!("Error: invalid index");
    //             std::process::exit(exitcode::DATAERR);
    //         }

    //         list[index_arg].data.as_mut().unwrap().append(&mut vec![Todo {
    //             data: command_args.string.to_string(),
    //             state: TaskState::Pending 
    //         }]);
    //     }
    // }
    // else if config.remote_location.is_empty() {
    //     println!("Error: no remote_location is set");
    //     std::process::exit(exitcode::CONFIG);
    // }
    // else{
    //     todo!();
    }

    // data::List::write(list, &config.local_location);
    return list;
}