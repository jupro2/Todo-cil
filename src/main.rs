mod task;
mod storage;

use clap::{Parser, Subcommand};
use task::Task;
use storage::{load_tasks, save_tasks};

extern crate colored;

use colored::Colorize;
use todo_cil::priority_color;

#[derive(Parser)]
#[command(name = "TODO")]
#[command(about = "This is a simple command-line task manager built with Rust")]
//--help


struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            let id = (tasks.len() as u32) + 1;
            let new_task = Task::new(id, description);
            tasks.push(new_task);
            save_tasks(&tasks);
            println!("{}","Task have add!".green());
            
        }

        //在添加的时候，如果有优先级数字，则将优先级数字加入。

        // Commands::List => {
        //     if tasks.is_empty() {
        //         println!("{}","📭 No tasks yet!".red());
        //     } else {
        //         for task in &tasks {
        //             println!(
        //                 "{}. [{}] {} ({})",
        //                 task.id,
        //                 if task.completed { "x".red() } else { " ".white() },
        //                 task.description,
        //                 task.created_at.format("%Y-%m-%d %H:%M:%S")
        //             );
        //         }
        //     }
        // }
        Commands::List=>{
            if tasks.is_empty() {
                println!("{}","没有任务".red());
            }else{
                tasks.sort_by_key(|tasks|tasks.priority);
                for task in &tasks{
                    println!(
                        "{}",
                        priority_color(task.priority)(&format!(
                            "{}. [{}] {} ({})",
                             task.id,
                            if task.completed { "x" } else { " " },
                            task.description,
                            task.created_at.format("%Y-%m-%d %H:%M:%S")
                        ))

                    )
                    
                }
            }
        }


        //更改展示，如果优先级更高，则用红色展示，依次是橙色，蓝色，灰色。

        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
                save_tasks(&tasks);
                println!("🎉 Task {} marked as done!", id);
            } else {
                println!("⚠️ Task not found!");
            }
        }

        Commands::Delete { id } => {
            tasks.retain(|t| t.id != id);
            save_tasks(&tasks);
            println!("🗑️ Task {} deleted!", id);
        }
    }

}




