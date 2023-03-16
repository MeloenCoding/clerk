use colored::Colorize;

use crate::{data::{TaskState, Todo, MainTaskFormat}, config::Config, CommandOutput, create_error};

// use termsize;

pub fn draw_cli(output: &CommandOutput, config: &Config) {
    let page_num: i64 = output.page_num.unwrap_or(0);
    let list: &Vec<MainTaskFormat> = output.data;
    let mut total_tasks: i64 = list.len().try_into().unwrap();
    let total_main_tasks: i64 = list.len().try_into().unwrap();
    let page_size: i64 = config.page_size as i64;
    let mut total_sub_tasks: i64 = 0;
    let mut total_completed: i64 = 0;
    let mut total_doing: i64 = 0;
    let mut total_pending: i64 = 0;

    for todo_item in list {
        let state: &TaskState = &todo_item.state;
        let data: &Vec<Todo> = todo_item.data.as_ref();
        match state {
            TaskState::Completed => {
                total_completed += 1;
            },
            TaskState::Doing => {
                total_doing += 1; 
            },
            TaskState::Pending => {
                total_pending += 1;
            }
        }
        for sub_task in data {
            match sub_task.state {
                TaskState::Completed => {
                    total_completed += 1;
                },
                TaskState::Doing => {
                    total_doing += 1; 
                },
                TaskState::Pending => {
                    total_pending += 1;
                }
            }
            total_sub_tasks += 1;
        }
    }
    total_tasks += total_sub_tasks;
    
    let mut max_page: i64 = (total_main_tasks + (page_size - 1)) / page_size;

    if max_page == 0 {
        max_page += 1;
    }

    if page_num >= max_page && list.len() != 0 {
        create_error("page_num out of bounds", Some(exitcode::DATAERR));
    }
    
    println!(
        "╔═════════╡ {} {}{} ╞═══════════╡ {}{}{}{}{}",
        "clerk".bright_cyan(),
        "v".purple(),
        env!("CARGO_PKG_VERSION").yellow(),
        
        "(".italic().bright_black(),
        (page_num + 1).to_string().italic().bright_black(),
        "/".italic().bright_black(),
        max_page.to_string().italic().bright_black(),
        ")".italic().bright_black()
    );
    println!("║ ");
    let start: i64 = page_num * page_size;
    for i in start as usize..list.len() {
        let todo_item: &MainTaskFormat = &list[i];
        let state: &TaskState = &todo_item.state;
        let data: &Vec<Todo> = todo_item.data.as_ref();

        match state {
            TaskState::Completed => {
                println!("║   [{}] {} {} ", i.to_string(), "◆ ".green(), &todo_item.title.strikethrough().bright_black());
                // println!("║  ╰─╴ ");
            },
            TaskState::Doing => {
                println!("║   [{}] {} {}", i.to_string(), "❖ ".blue(), &todo_item.title);
                // println!("║  ╰─╴ ", );
            },
            TaskState::Pending => {
                println!("║   [{}] ◇  {}", i.to_string(), &todo_item.title);
                // println!("║  ╭ [{}] [ ] {}", i.to_string(), todo_item.title.as_ref().unwrap());
                // println!("║  ╰─╴" );
            }
        }
        let mut ii: i32 = 0;
        for sub_task in data {
            match sub_task.state {
                TaskState::Completed => {
                    println!("║           ╰─ [{}] {} {}", ii, "◆ ".green(), sub_task.data.strikethrough().bright_black());
                }, 
                TaskState::Doing => {
                    println!("║           ╰─ [{}] {} {}", ii,"❖ ".blue(), sub_task.data);
                },
                TaskState::Pending => {
                    println!("║           ╰─ [{}] ◇  {}", ii, sub_task.data);
                }
            }
            // println!("║            │");
            ii += 1;
        }
        if i as i64 == start + page_size - 1 {
            break;
        }
        println!("║ ");
    }

    let completion: i64 = ((total_completed as f64 / total_tasks as f64) * 100.0) as i64;
    println!("║ ");

    if list.len() != 0 {
        println!("║      {}", format!("{}% of all tasks are completed", completion).bright_black());
        println!("║  ─ {} completed · {} doing · {} pending ─", total_completed.to_string().bright_green(), total_doing.to_string().bright_cyan(), total_pending.to_string().bright_magenta());
    }
    else {
        println!("║ ─ run '{}' for more information about how to add tasks ─", format!("clerk.exe -h").bold());
    }
    println!("╨ ");
}
