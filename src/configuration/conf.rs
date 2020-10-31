use amethyst::input::{Bindings, StringBindings};
use chrono::{DateTime, Utc};


pub struct User {
    name: String,
    bindings: Bindings<StringBindings>,
    save_files: Vec<Save_file>,
}

pub struct Save_file {
    name: String,
    time: DateTime<Utc>,
    user_name: String,
}