mod configuration

use amethyst_input::Bindings
use time::{date , time}


pub struct User {
    name: String,
    bindings: Bindings,
    save_files: Vec<Save_file>,
}

pub struct Save_file {
    name: String,
    date: Date,
    time: Time,
    user_name: String,
}