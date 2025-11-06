use crate::priority_color;
use crate::storage::save_tasks;
use crate::task::Task;
use colored::Colorize;

pub fn add(tasks: &mut Vec<Task>, description: String, priority: Option<u8>) {
    let id = (tasks.len() as u32) + 1;
    let priority_not_null = priority.unwrap_or(1);
    let new_task = Task::new(id, description, priority_not_null);
    tasks.push(new_task);
    save_tasks(&tasks);
    println!("{}", "Task have add!".green());
}

//Argms:
//tasks 文件流,descdescription 描述，priority 优先级

pub fn list(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("{}", "No tasks".red());
    } else {
        tasks.sort_by_key(|task1| u8::MAX - task1.priority);

        for task in tasks {
            if task.completed == false {
                println!(
                    "{}",
                    priority_color(task.priority)(&format!(
                        "{}. [{}] {} ({})",
                        task.id,
                        " ",
                        task.description,
                        task.created_at.format("%Y-%m-%d %H:%M:%S")
                    ))
                )
            }
        }
    }
}
// 2025年11月6日15:31:05 更改list，只显示未完成的任务，不再显示已经完成的任务

pub fn done(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        save_tasks(&tasks);
        println!("🎉 Task {} marked as done!", id);
    } else {
        println!("⚠️ Task not found!");
    }
}

pub fn delete(tasks: &mut Vec<Task>, id: u32) {
    tasks.retain(|t| t.id != id);
    save_tasks(&tasks);
    println!("🗑️ Task {} deleted!", id);
}

pub fn edit(tasks: &mut Vec<Task>, id: u32, new_description: String) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.description = new_description;
        save_tasks(&tasks);
        println!("Task {} updated!", id);
    } else {
        println!("Task not found!");
    }
}

pub fn search(tasks: &mut Vec<Task>, search_term: String) {
    let mut results = Vec::new();

    for task in tasks {
        if task.description.contains(&search_term) {
            results.push(task.clone());
        }
    }

    if results.is_empty() {
        println!("{}", "No matching tasks".red());
    } else {
        results.sort_by_key(|task1| u8::MAX - task1.priority);

        for task in results {
            if task.completed == false {
                println!(
                    "{}",
                    priority_color(task.priority)(&format!(
                        "{}. [{}] {} ({})",
                        task.id,
                        " ",
                        task.description,
                        task.created_at.format("%Y-%m-%d %H:%M:%S")
                    ))
                )
            }
        }
    }
}
