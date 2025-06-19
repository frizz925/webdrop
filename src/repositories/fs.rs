use std::{fs::File, io::{Read, Write}, path::Path};

use serde::{de::DeserializeOwned, Serialize};

use super::Result;

pub trait BaseFsRepository {
    fn save<T: Serialize>(&self, mut file: File, data: &T) -> Result<()> {
        let json = serde_json::to_string(data)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn load<P: AsRef<Path>, T: DeserializeOwned>(&self, path: P) -> Result<T> {
        let mut file = File::open(path)?;
        let data = serde_json::from_reader(&mut file)?;
        Ok(data)
    }

    fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn read_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let mut buf = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
}