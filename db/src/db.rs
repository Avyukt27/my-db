use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Seek, Write},
    path::{Path, PathBuf},
};

use crate::{db_error::DbError, db_types::DataType};

#[derive(Debug)]
pub struct Database {
    path: PathBuf,
    data: HashMap<String, DataType>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DbError> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let data = Self::read_file_to_hashmap(&mut file)?;
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            data: data,
        })
    }

    pub fn get<K: AsRef<str>>(&self, key: K) -> Result<&DataType, DbError> {
        let key = key.as_ref();
        self.data
            .get(key)
            .ok_or_else(|| DbError::KeyNotFound(key.to_owned()))
    }

    pub fn set<K: AsRef<str>, V: Into<DataType>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<(), DbError> {
        let key = key.as_ref();
        let value = value.into();

        let new_line = format!("{}: {}\n", key, value.to_str().into_owned());
        let mut updated_file = File::options().append(true).open(&self.path)?;
        updated_file.write_all(new_line.as_bytes())?;
        self.data.insert(key.to_string(), value);

        Ok(())
    }

    pub fn remove<K: AsRef<str>>(&mut self, key: K) -> Result<Option<DataType>, DbError> {
        let key = key.as_ref();

        let new_line = format!("{}: __DELETED__\n", key);
        let mut updated_file = File::options().append(true).open(&self.path)?;
        updated_file.write_all(new_line.as_bytes())?;
        Ok(self.data.remove(key))
    }

    pub fn compact(&self) -> Result<(), DbError> {
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        for (k, v) in self.data.iter() {
            let line = format!("{}: {}\n", k, v.to_str());
            file.write_all(line.as_bytes())?;
        }

        Ok(())
    }

    pub fn keys(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.data.keys().map(|k| k.as_str()).collect();
        keys.sort();
        keys
    }

    pub fn exists<K: AsRef<str>>(&self, key: K) -> bool {
        let key = key.as_ref();
        self.data.get(key).is_some()
    }

    fn read_file_to_hashmap(file: &mut File) -> Result<HashMap<String, DataType>, DbError> {
        file.seek(io::SeekFrom::Start(0))?;
        let lines = io::BufReader::new(file).lines();
        let mut data: HashMap<String, DataType> = HashMap::new();

        for line in lines {
            let line = line?;
            if let Some((k, v)) = line.split_once(": ") {
                if v == "__DELETED__" {
                    data.remove(&k.to_owned());
                } else {
                    let key = k.to_owned();
                    let value = v.parse::<DataType>().unwrap();
                    data.insert(key, value);
                }
            }
        }

        Ok(data)
    }
}
