use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Seek, Write},
    path::{Path, PathBuf},
};

use crate::db_types::DataType;

#[derive(Debug)]
pub struct DataBase {
    path: PathBuf,
    data: HashMap<String, DataType>,
}

impl DataBase {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let data = read_file_to_hashmap(&mut file)?;
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            data: data,
        })
    }

    pub fn get(&self, key: &str) -> Option<&DataType> {
        self.data.get(key)
    }

    pub fn set<T: Into<DataType>>(&mut self, key: &str, value: T) -> io::Result<()> {
        let value = value.into();
        let new_line = format!("{}: {}\n", key, value.to_str().into_owned());
        let mut updated_file = File::options().append(true).open(&self.path)?;
        updated_file.write_all(new_line.as_bytes())?;
        self.data.insert(key.to_string(), value);

        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> io::Result<Option<DataType>> {
        let new_line = format!("{}: __DELETED__\n", key);
        let mut updated_file = File::options().append(true).open(&self.path)?;
        updated_file.write_all(new_line.as_bytes())?;
        Ok(self.data.remove(key))
    }
}

fn read_file_to_hashmap(file: &mut File) -> io::Result<HashMap<String, DataType>> {
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
