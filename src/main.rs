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

struct AppState {
    db: Mutex<Database>
}

async fn create_task (app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new()
    };

    let data = web::Data::new(AppState {
        db: Mutex::new(db)
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|oring, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["POST", "GET", "PUT", "DELETE"])
                    .allowed_header(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)

            )
            .app_data(data.clone())
            .route("/task", web::post()).to(create_task)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
