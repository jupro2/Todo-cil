use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}


impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
            created_at: Utc::now(),
        }
    }
}