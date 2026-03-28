use chrono::Utc;
use std::fs;
use std::collections::HashMap;
use crate::task::{Task, Status};

pub struct TodoManager { 
    pub storage: HashMap<u32, Task>,
    pub file: Option<String>,
}

impl TodoManager {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            file: None,
        }
    }

    pub fn init(&mut self, file: &str) {
        if self.file.is_none() {
            self.file = Some(file.to_string());
        }

        let content = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("Failed to read {file}: {e}"));

        self.storage = serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {file}: {e}"));
    }

    pub fn add_task(&mut self, task: Task) {
        let id = self.storage.keys().max().copied().unwrap_or(0) + 1;
        self.storage.insert(id, task);
    }

    pub fn save(&self) {
        if let Some(path) = &self.file {
            let json = serde_json::to_string_pretty(&self.storage)
                .expect("Failed to serialize!");

            fs::write(path, json)
                .expect("Failed to write file!");
        }
    }

    pub fn show_tasks(&self) {
        let mut keys: Vec<u32> = self.storage.keys().copied().collect();
        keys.sort();
        for id in keys {
            println!("{id} {}", self.storage[&id]);
        }
    }

    pub fn done(&mut self, id: u32) {
        if let Some(task) = self.storage.get_mut(&id) {
            task.status = Status::Done;
            task.updated_at = Utc::now();
        } else {
            println!("Task {id} not found!");
        }
    }

    pub fn delete(&mut self, id: u32) {
        if self.storage.remove(&id).is_none() {
            println!("Task {id} not found!");   
        }
    }
}

