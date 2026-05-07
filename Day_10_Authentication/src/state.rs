use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::model::User;

pub type Db = Arc<Mutex<HashMap<String, User>>>;