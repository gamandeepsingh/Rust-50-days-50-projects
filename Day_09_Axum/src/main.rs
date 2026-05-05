use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// ===== Data Model =====

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// ===== App State =====

type Db = Arc<Mutex<HashMap<u32, Todo>>>;

// ===== Main =====

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(db)
        .layer(middleware::from_fn(log_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

// ===== Handlers =====

// GET /todos
async fn get_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    let todos = db.lock().unwrap();
    Json(todos.values().cloned().collect())
}

// GET /todos/:id
async fn get_todo(Path(id): Path<u32>, State(db): State<Db>) -> impl IntoResponse {
    let todos = db.lock().unwrap();

    match todos.get(&id) {
        Some(todo) => (StatusCode::OK, Json(todo.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, "Todo not found").into_response(),
    }
}

// POST /todos
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

async fn create_todo(
    State(db): State<Db>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let mut todos = db.lock().unwrap();

    let id = todos.len() as u32 + 1;

    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };

    todos.insert(id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

// PUT /todos/:id
async fn update_todo(
    Path(id): Path<u32>,
    State(db): State<Db>,
) -> impl IntoResponse {
    let mut todos = db.lock().unwrap();

    match todos.get_mut(&id) {
        Some(todo) => {
            todo.completed = true;
            (StatusCode::OK, Json(todo.clone())).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Todo not found").into_response(),
    }
}

// DELETE /todos/:id
async fn delete_todo(
    Path(id): Path<u32>,
    State(db): State<Db>,
) -> impl IntoResponse {
    let mut todos = db.lock().unwrap();

    if todos.remove(&id).is_some() {
        (StatusCode::OK, "Deleted").into_response()
    } else {
        (StatusCode::NOT_FOUND, "Todo not found").into_response()
    }
}

// ===== Middleware (logging) =====

async fn log_middleware(req: axum::http::Request<axum::body::Body>, next: Next) -> Response {
    println!("➡️ {} {}", req.method(), req.uri());
    next.run(req).await
}