use amethyst::input::{Bindings, StringBindings};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    bindings: Bindings<StringBindings>,
    save_files: Vec<Save_file>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Save_file {
    name: String,
    time: DateTime<Utc>,
    user_name: String,
}