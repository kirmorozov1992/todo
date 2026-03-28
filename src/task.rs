use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Todo,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,    
}

impl Task {
    pub fn new(name: &str) -> Self {
        let now = Utc::now();
        Self {
            name: name.to_string(),
            status: Status::Todo,
            created_at: now,
            updated_at: now,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "[{:?}] {} (created: {})",
            self.status,
            self.name,
            self.created_at.format("%Y-%m-%d %H:%M"),
        )
    }
}

