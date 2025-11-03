pub mod task;
pub mod storage;
pub mod commands;
use colored::Colorize;





pub fn priority_color(priority: u8) -> impl Fn(&str) -> String {
    match priority {
        1 => |s: &str| s.green().to_string(),
        2 => |s: &str| s.yellow().to_string(),
        3 => |s: &str| s.blue().to_string(),
        4 => |s: &str| s.red().to_string(),
        _ => |s: &str| s.white().to_string(),
    }
}