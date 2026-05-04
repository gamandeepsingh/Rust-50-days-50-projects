use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get,
    Create,
    Update { id: u32 },
    Delete { id: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    user_id: u32,
    id: Option<u32>, // server assigns
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get => get_todos().await.unwrap(),
        Commands::Create => create_todo().await.unwrap(),
        Commands::Update { id } => update_todo(id).await.unwrap(),
        Commands::Delete { id } => delete_todo(id).await.unwrap(),
    }
}


async fn get_todos() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/todos")
        .await?;

    let todos: Vec<Todo> = res.json().await?;

    for (i, t) in todos.iter().take(5).enumerate() {
        println!("{}: {:?}", i, t);
    }

    Ok(())
}

async fn create_todo() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "Learn Rust".to_string(),
        completed: false,
    };

    let res = client
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?;

    let created: Todo = res.json().await?;
    println!("Created: {:?}", created);

    Ok(())
}


async fn update_todo(id: u32) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let updated = Todo {
        user_id: 1,
        id: Some(id),
        title: "Updated title".to_string(),
        completed: true,
    };

    let res = client
        .put(&format!("https://jsonplaceholder.typicode.com/todos/{}", id))
        .json(&updated)
        .send()
        .await?;

    let todo: Todo = res.json().await?;
    println!("Updated: {:?}", todo);

    Ok(())
}


async fn delete_todo(id: u32) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client
        .delete(&format!("https://jsonplaceholder.typicode.com/todos/{}", id))
        .send()
        .await?;

    println!("Deleted status: {}", res.status());

    Ok(())
}