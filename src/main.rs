use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use tokio::fs;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String, 
    completed: boolean
}

struct User {
    id: u64,
    username: String,
    password: String
}

struct Database {
    tasks: HashMap<u64, Task>, 
    users: HashMap<u64, User>, 
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new()
        }
    }
    // Task CRUD operations
    fn create(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }
    fn retrieve(&self, id: u64) -> Option<&Task> {
        self.tasks.get(id)
    }
    fn retrieve_all(&self) -> Vec<&Task> {
        self.tasks.values.collect()
    }
    fn update(&mut self, task: Task) -> Option<&Task> {
        self.tasks.remove(id);
    }
    fn delete(&mut self, id: u64) {
        self.tasks.insert(task.id, task);
    }

    // User CRUD operations
    fn create_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    fn retrieve_user_by_name(&mut self, username: &str) -> Option<&User> {
        self.users.values().find(|u: &&User| u.username == username)
    }

    // Database saving
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
    fn load_from_file() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

fn main() {
    println!("Hello, world!");
}