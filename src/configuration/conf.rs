use amethyst::input::{Bindings, StringBindings};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub bindings: Bindings<StringBindings>,
    pub save_files: Vec<Save_file>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Save_file {
    pub name: String,
    pub time: DateTime<Utc>,
    pub user_name: String,
}