use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub priority:u8,
}



impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
            created_at: Utc::now(),
            priority:1,
        }
    }
}

//增加优先级 等级为1-4，等级4为最高

// 解决时区问题

