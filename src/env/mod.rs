use envfile::EnvFile;
use std::env;

pub struct EnvComponent {}

impl EnvComponent {
    pub fn getData(&mut self) -> EnvFile {
        let path: std::path::PathBuf = env::current_dir().unwrap();
        let pathFileEnv: String = format!("{}\\.env", path.display());

        println!("{}", pathFileEnv);

        EnvFile::new(pathFileEnv).expect("no file .env")
    }
}

pub static mut envComponentSingleton: EnvComponent = EnvComponent {};
