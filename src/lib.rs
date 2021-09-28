use std::{collections::HashMap, io::Read, ops::Index};
use thiserror::Error;

// TODO: Special type for value that automatically finds substitute fields
/// Holds a key-value pairs for locale entries from locale file
#[derive(Debug)]
pub struct LocaleFile {
    /// Key: locale key (section.protperty)
    /// Item: Locale string
    pub entries: HashMap<String, String>
}

impl Index<&String> for LocaleFile {
    type Output = String;

    fn index(&self, key: &String) -> &Self::Output {
        self.entries.index(key)
    }
}

impl LocaleFile {
    /// Create LocaleFile instance from reader
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, LocaleError> {
        let mut buf = String::new();

        reader.read_to_string(&mut buf).map_err(|_| LocaleError::FilesystemError)?;

        let lines: Vec<&str> = buf.as_str().split('\n').collect();
        let mut current_section = "";
        let mut result = Self{entries: HashMap::new()};
        for line in lines {
            if line.starts_with("[") && line.ends_with("]") {
                current_section = &line[1..line.len()-2];
            } else if let Some(sep_idx) = line.find("=") {
                let mut key = String::from(current_section);
                key.push('.');
                key.push_str(&line[0..sep_idx]);
                let value = &line[sep_idx+1..line.len()-1];
                result.entries.insert(key, value.into());
            } else if line != "\n" {
                return Err(LocaleError::InvalidLocaleFile)
            }
        }
        Ok(result)
    }
}

#[derive(Debug, Clone, Error)]
pub enum LocaleError {
    #[error("Invalid locale file")]
    InvalidLocaleFile,
    #[error("Filesystem error")]
    FilesystemError
}
