pub mod conf;

use log;
use std::path::{Path, PathBuf};
use conf::{User, Save_file};
use std::fs::File;
use std::fs;
use std::io::{prelude::*,Error};

use std::io;
use ron::de as de_ron;
use ron::ser as ser_ron;

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
        Configurator::default();
        self.get_conf_files(&self.folder);
        *self
    }

    pub fn load(&self, folder: &str) {
        log::info!("Load configurator");
        self.folder = folder.to_string();
        self.get_conf_files(&self.folder);
    }

    pub fn insert_user(&self, usr: User) {
        let usr_str = ser_ron::to_string(&usr);
        let usr_file = PathBuf::from(self.folder).join(format!("{}.cfg", usr.name));
        std::fs::write(usr_file,usr_str.unwrap());

        log::info!("User {} was insert in {}",usr.name,self.folder);

    }

    pub fn delete_user(&self, usr: User) -> Result<(),io::Error> {
        let dir = Path::new(&self.folder).join(format!("{}.cfg",usr.name));
        fs::remove_file(dir)
    }

    fn get_conf_files(&self, folder: &str) -> Result<Vec<User>, io::Error> {

        let path = Path::new(folder);
        let files: Vec<User> = Vec::new();

        let entries = path.read_dir()?.map(|file_path| {
            file_path.map(|res| {
                res.path()})
            }).collect::<Result<Vec<_>, io::Error>>()?;

        entries.iter().filter(|res| {

            let extension = res.extension().unwrap().to_str().unwrap();
            match extension {
                "cfg" => true,
                "conf" => true,
                _ => false,
            }
        }).for_each(|p| {
            let file = File::open(p).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let user_file: User = de_ron::from_str(content.as_str()).unwrap();
            files.push(user_file);
        });

        Ok(files)
    }
}
