use serde::{ Serialize };

// Define struct
#[derive(Debug, Serialize)]
struct User {
    name: String,
    age: u32,
    is_dev: bool,
    city: Option<String>,
}

fn main() {
    let user = User {
        name: String::from("Gamandeep Singh"),
        age: 24,
        city: Some(String::from("Rust land")),
        is_dev: true,
    };
    //  Struct -> JSON
    let json = serde_json::to_string_pretty(&user).unwrap();

    println!("{}", json);
}
