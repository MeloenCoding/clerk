use colored::{Colorize, control::SHOULD_COLORIZE};

use crate::{data::{TaskState, Todo, MainTaskFormat, ListData}, config::Config, CommandOutput, create_error};

pub struct CalculatedTasks {
    pub max_page: i64,
    pub max_page_index: i64,
    pub total_tasks: i64,
    pub total_main_tasks: i64,
    pub page_size: i64, 
    pub total_sub_tasks: i64,
    pub total_completed: i64,
    pub total_doing: i64,
    pub total_pending: i64
}

pub fn calculate_changed_page(list: &ListData, config: &Config, index_of_changed_task: i64) -> Option<i64> {
    let list_info: CalculatedTasks = calculate_tasks(list, config);
    
    let changed_page_num = Some((index_of_changed_task as i64 + (list_info.page_size - 1)) / list_info.page_size - 1);
    return changed_page_num;
}

pub fn calculate_tasks(data: &ListData, config: &Config) -> CalculatedTasks {
    let total_main_tasks: i64 = data.len().try_into().unwrap();
    let page_size: i64 = config.page_size as i64;
    let mut total_tasks: i64 = data.len().try_into().unwrap();
    let mut total_sub_tasks: i64 = 0;
    let mut total_completed: i64 = 0;
    let mut total_doing: i64 = 0;
    let mut total_pending: i64 = 0;

    for todo_item in data {
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

    let max_page_index: i64 = max_page - 1;

    return CalculatedTasks {
        max_page,
        max_page_index,
        total_tasks,
        total_main_tasks,
        page_size,
        total_sub_tasks,
        total_completed,
        total_doing,
        total_pending,
    };

}

pub fn draw_cli(output: &CommandOutput, config: &Config) {
    let list: &Vec<MainTaskFormat> = output.data;

    let mut page_num: i64 = output.page_num.unwrap_or(0);
    
    let list_info: CalculatedTasks = calculate_tasks(output.data, config);
    
    if page_num < 0 {page_num = 1;}

    if page_num >= list_info.max_page && list.len() != 0 {
        create_error("page_num out of bounds", Some(exitcode::DATAERR));
    }

    SHOULD_COLORIZE.set_override(!config.color_blind);

    println!(
        "╔═════════╡ {} {}{} ╞═══════════╡ {}{}{}{}{}",
        "clerk".bright_cyan(),
        "v".purple(),
        env!("CARGO_PKG_VERSION").yellow(),
        
        "(".italic().bright_black(),
        (page_num + 1).to_string().italic().bright_black(),
        "/".italic().bright_black(),
        list_info.max_page.to_string().italic().bright_black(),
        ")".italic().bright_black()
    );
    println!("║ ");
    let start: i64 = page_num * list_info.page_size;
    for i in start as usize..list.len() {
        let todo_item: &MainTaskFormat = &list[i];
        let state: &TaskState = &todo_item.state;
        let data: &Vec<Todo> = todo_item.data.as_ref();

        match state {
            TaskState::Completed => {
                println!("║   [{}] {} {} ", i.to_string(), if config.use_unicode {"√ ".green()} else {"◆ ".green()}, &todo_item.title.strikethrough().bright_black());
            },
            TaskState::Doing => {
                println!("║   [{}] {} {}", i.to_string(), (if config.use_unicode {"■ "} else {"❖ "}).blue(), &todo_item.title);
            },
            TaskState::Pending => {  
                println!("║   [{}] {} {}", i.to_string(), (if config.use_unicode {"o "} else {"◇ "}), &todo_item.title);
            }
        }
        let mut ii: i32 = 0;
        for sub_task in data {
            match sub_task.state {
                TaskState::Completed => {
                    println!("║           ╰─ [{}] {} {}", ii, (if config.use_unicode {"√ "} else {"◆ "}).green(), sub_task.data.strikethrough().bright_black());
                }, 
                TaskState::Doing => {
                    println!("║           ╰─ [{}] {} {}", ii, (if config.use_unicode {"■ "} else {"❖ "}).blue(), sub_task.data);
                },
                TaskState::Pending => {
                    println!("║           ╰─ [{}] {} {}", ii, (if config.use_unicode {"o "} else {"◇ "}), sub_task.data);
                }
            }
            ii += 1;
        }
        if i as i64 == start + list_info.page_size - 1 {
            break;
        }
        println!("║ ");
    }

    let completion: i64 = ((list_info.total_completed as f64 / list_info.total_tasks as f64) * 100.0) as i64;
    println!("║ ");

    if list.len() != 0 {
        println!("║      {}", format!("{}% of all tasks are completed", completion).bright_black());
        println!("║  ─ {} completed · {} doing · {} pending ─", list_info.total_completed.to_string().bright_green(), list_info.total_doing.to_string().bright_cyan(), list_info.total_pending.to_string().bright_magenta());
    }
    else {
        println!("║ ─ run '{}' for more information about how to add tasks ─", format!("clerk.exe -h").bold());
    }
    println!("╨ ");
}
