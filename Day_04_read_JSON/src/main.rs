use std::fs;
use serde::{ Deserialize };

// Define json type
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
    is_dev: bool,
    city: Option<String>,
}

fn main() {
    //1. Single JSON Object
    // read file
    let data = fs::read_to_string("data.json")
        .expect("Unable to read file");
    println!("\nDATA: {:?}\n", data); // string

    // parse JSON → struct
    let user: User = serde_json::from_str(&data)
        .expect("JSON parse error");

    println!("{:?}", user);  // access value using user.age or user.name


    //2. Array of JSON
    let data2 = fs::read_to_string("array-data.json")
                .expect("Unable to read file");

    let users: Vec<User> = serde_json::from_str(&data2)
        .expect("JSON parse error");

    println!("\nArrays of JSON: \n");
    
    for (i, u) in users.iter().enumerate() {
        println!("{}: {:?}", i, u);
    }
}