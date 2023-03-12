use colored::Colorize;

use crate::data::{ListData, TaskState, Todo};

pub fn draw_cli(list: &ListData) {
    let mut i: usize = 0;
    let mut total_completed: usize = 0;

    println!(
        "╔═════════╡ {} {}{} ╞═════╡ {}{}{}{}{}",
        "clerk".bright_cyan().blink(),
        "v".purple(),
        env!("CARGO_PKG_VERSION").yellow(),

        "(".italic().bright_black(),
        "1".italic().bright_black(),
        "/".italic().bright_black(),
        "2".italic().bright_black(),
        ")".italic().bright_black()
    );
    println!("║ ");
    for todo_item in list {
        let state: &TaskState = &todo_item.state;
        let data: &Vec<Todo> = todo_item.data.as_ref();

        match state {
            TaskState::Completed => {
                println!("║   [{}] {} {} ", i.to_string(), "◆ ".green(), &todo_item.title.strikethrough());
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
                    println!("║           ╰─ [{}] {} {}", ii, "◆ ".green(), sub_task.data.strikethrough());
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
        
        println!("║ ");
        i += 1;
    }
    println!("╨ ");
    // println!("╚══════════════════════════════════╡");
}