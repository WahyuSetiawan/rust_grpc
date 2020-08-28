use envfile::EnvFile;
use std::path::Path;

pub struct EnvComponent {}

impl EnvComponent {
    pub fn getData(&mut self) -> EnvFile {
        return EnvFile::new(&Path::new(".env")).unwrap();
    }
}

pub static mut envComponentSingleton: EnvComponent = EnvComponent {};
