mod configuration::*

use log;
use std::path

pub struct Configurator{
    folder: Path,
    profiles_users: vec<User>,
}

impl Default for Configurator {
    fn default() -> Self {
        Configurator{
            folder: Path::default(),
            profiles_users: Vec::new(),
        }
    }
}

pub impl Configurator {

    new(&self) -> Self {
        Configurator{
            folder: Path::Default(),
            profiles_users: Vec::new(),
        }
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