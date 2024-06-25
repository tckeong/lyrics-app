use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::Write;

pub struct Utils {
    path: String,
}

impl Utils {
    pub fn new() -> Self {
        Utils {
            path: "./data".to_string(),
        }
    }

    fn create_folder_if_not_exists(&self) -> Result<&Self, Box<dyn Error>> {
        let folder_path = self.path.as_str();
        if !fs::metadata(folder_path).is_ok() {
            fs::create_dir(folder_path)?;
        }

        Ok(self)
    }

    fn create_file(&self, file_path: String) -> Result<File, Box<dyn Error>> {
        let file = File::create(file_path)?;
        Ok(file)
    }

    fn hash_utf8(data: &str) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write(data.as_bytes());
        hasher.finish().to_string()
    }

    pub async fn save_lrc(&self, title: String, lyrics: String) -> Result<(), Box<dyn Error>> {
        let file_name = Self::hash_utf8(&title);
        let file_path = format!("{}/{}.lrc", self.path, file_name);
        let mut file = self.create_folder_if_not_exists()?.create_file(file_path)?;
        file.write(lyrics.as_bytes())?;

        Ok(())
    }
}
