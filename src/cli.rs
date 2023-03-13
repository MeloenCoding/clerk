use colored::Colorize;

use crate::data::{ListData, TaskState, Todo};

// use termsize;

pub fn draw_cli(list: &ListData, page_num: Option<u16>) {
    let page_num = page_num.unwrap_or(0);
    let mut total_tasks: u32 = list.len().try_into().unwrap();
    let total_main_tasks: u32 = list.len().try_into().unwrap();
    let mut total_sub_tasks: u32 = 0;
    let mut total_completed: u32 = 0;
    let mut total_doing: u32 = 0;
    let mut total_pending: u32 = 0;

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

    let mut max_page: f64 = (<u32 as Into<f64>>::into(total_main_tasks)/4.0_f64).ceil();

    if max_page == 0.0 {
        max_page += 1.0;
    }

    if page_num >= max_page as u16 && list.len() != 0 {
        println!("Error: page_num out of bounds");
        std::process::exit(exitcode::DATAERR);
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
    let start: usize = (page_num * 5) as usize;
    for i in start..list.len() {
        let todo_item = &list[i];
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
        if i == start + 4 {
            break;
        }
        println!("║ ");
    }

    let completion: f64 = <u32 as Into<f64>>::into(total_completed) / <u32 as Into<f64>>::into(total_tasks) * 100.0;
    println!("║ ");

    if list.len() != 0 {
        println!("║      {}", format!("{}% of all tasks are completed", completion.floor()).bright_black());
        println!("║  ─ {} completed · {} doing · {} pending ─", total_completed.to_string().bright_green(), total_doing.to_string().bright_cyan(), total_pending.to_string().bright_magenta());
    }
    else {
        println!("║ ─ run '{}' for more information about how to add tasks ─", format!("clerk.exe -h").bold());
    }
    println!("╨ ");
}


// termsize::get().map(|size| {
//     let mut test = "╞".to_owned();
//     for _i in 0..size.cols-2 {
//         test.push('═');
//     }
//     test.push('╡');
//     // println!("{}", test);
// });