pub mod conf;

use log;
use std::path::Path;
use conf::{User, Save_file};
use serde::{Serialize, Deserialize};

pub struct Configurator{
    folder: String,
    profiles_users: Vec<User>,
}

impl Default for Configurator {
    fn default() -> Self {
        Configurator{
            folder: String::from("./assets/config/conf"),
            profiles_users: Vec::new(),
        }
    }
}

impl Configurator {

    pub fn new(&self) -> Self {
        Configurator::default()
    }

    pub fn load(&self, folder: &str) {
        log::info!("Load configurator")
        self.folder = folder.to_string();


    }

    pub fn insert_user(&self, usr: User) -> Result<User> {

    }

    pub fn delete_user(&self, usr: User) -> Result<User> {

    }

    pub fn load_file(&self, file: Path) -> Result<Save_file> {

    }

    fn get_conf_files(&self, folder: &str) -> Result<Vec<Save_file>> {

        let path = Path::new(folder);
        let files: Vec<Save_file> = Vec::new();
        for entry in path.read_dir().or(log::warn!("Configurator folder is empty in {:?}",folder) {
            if let Ok(entry) = entry {

                files.push(entry)
            } 
        }

        Ok(files)
    }
}
