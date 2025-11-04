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
// 优先级彩色输出 1->green, 2->yellow, 3->blue, 4->red
// 如果优先级为0，则输出灰色
