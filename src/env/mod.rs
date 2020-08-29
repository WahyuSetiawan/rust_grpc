use envfile::EnvFile;
use std::env;
use std::path::Path;

pub struct EnvComponent {}

impl EnvComponent {
    pub fn getData(&mut self) -> EnvFile {
        let path = env::current_dir().unwrap();
        let pathFileEnv = format!("{}\\.env", path.display());

        println!("{}", pathFileEnv);

        EnvFile::new(pathFileEnv).expect("no file .env")
    }
}

pub static mut envComponentSingleton: EnvComponent = EnvComponent {};
