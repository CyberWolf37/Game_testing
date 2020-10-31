pub mod conf;

use log;
use std::path::Path;
use conf::{User, Save_file};

pub struct Configurator{
    folder: Path,
    profiles_users: Vec<User>,
}

impl Default for Configurator {
    fn default() -> Self {
        Configurator{
            folder: Path::default(),
            profiles_users: Vec::new(),
        }
    }
}

impl Configurator {

    new(&self) -> Self {
        Configurator::default()
    }

    load(&self, folder: Path) {

    }

    insert_user(&self, usr: User) -> Result<User> {

    }

    delete_user(&self, usr: User) -> Result<User> {

    }

    load_file(&self, file: Path) -> Result<Save_file> {

    }
}
