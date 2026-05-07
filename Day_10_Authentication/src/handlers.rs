use axum::{Json, extract::State};
use crate::{model::User, state::Db, auth::hash_password};
use crate::auth::{verify_password, create_jwt};

pub async fn login(
    State(db): State<Db>,
    Json(user): Json<User>,
) -> Result<String, String> {
    let users = db.lock().unwrap();

    let existing_user = users.get(&user.username);

    match existing_user {
        Some(stored_user) => {
            let valid = verify_password(
                &user.password,
                &stored_user.password,
            );

            if valid {
                Ok(create_jwt(user.username))
            } else {
                Err("Invalid password".to_string())
            }
        }
        None => Err("User not found".to_string()),
    }
}


pub async fn register(
    State(db): State<Db>,
    Json(mut user): Json<User>,
) -> String {
    let mut users = db.lock().unwrap();

    user.password = hash_password(&user.password);

    users.insert(user.username.clone(), user);

    "User registered".to_string()
}