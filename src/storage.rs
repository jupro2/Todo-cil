use std::{fs, path::Path};
use serde_json;
use crate::task::Task;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE_PATH).exists() {
        return vec![];
    }
    let data = fs::read_to_string(FILE_PATH).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}
//1.创建json文件保存
//2.加载json文件放入内存

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE_PATH, data).expect("Unable to save tasks");
}
