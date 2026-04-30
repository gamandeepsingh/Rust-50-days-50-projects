use reqwest;
use serde::{Serialize, Deserialize};
use reqwest::Client;


#[allow(dead_code)] // to get rid of "unused variable" warning 
#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")] // map user_id -> userId 
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[derive(Serialize)]
struct Params {
    page: u32,
    limit: u32,
}

const PARAMS: Params = Params { page: 1, limit: 10 };

#[tokio::main] // tokio used to make function async
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let res = client
    .get("https://jsonplaceholder.typicode.com/posts")
    .header("Authorization","your jwt token")
    .query(&PARAMS)
    .send()
    .await?;

    let todos: Vec<Todo> = res.json().await?;

for (i, t) in todos.iter().enumerate() {
    println!("{}: {:?}", i, t);
}

    Ok(())
}