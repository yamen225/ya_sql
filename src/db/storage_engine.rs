use std::{collections::HashMap, fs::{File, OpenOptions}, io::Write, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use super::schema::{Table, Row};


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct StorageEngine{
    pub tables: HashMap<String, Table>
}

impl StorageEngine{
    pub fn new() -> Self{
        StorageEngine{
            tables: HashMap::new()
        }
    }

    pub fn create_table(&mut self, table_name: &str, columns: Vec<String>){
        self.tables.insert(table_name.to_string(), Table{
            columns: columns,
            rows: HashMap::new()
        });
    }

    // TODO: Implement Error if insert did not work
    pub fn insert_row(&mut self, table_name: &str, row: Row) {
        if let Some(table) = self.tables.get_mut(table_name) {
            let row_id = table.rows.len();
            table.rows.insert(row_id, row);
        }
    }

    pub fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error>{
        buffer.clear();
        buffer.extend(bincode::serialize(self).unwrap());
        Ok(())
    }

    pub fn deserialize(buffer: &[u8]) -> Result<Self, std::io::Error>{
        Ok(bincode::deserialize(buffer).unwrap())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileSystem {
    pub storage_engine: StorageEngine,
    pub file_path: String
}

impl FileSystem{
    pub fn new(file_path: &str) -> Self {
        let mut storage_engine = StorageEngine::new();
        if Path::new(file_path).exists(){
            storage_engine = FileSystem::load_from_file(file_path).unwrap_or(StorageEngine::new());
        }
        FileSystem{
            storage_engine,
            file_path: file_path.to_string()
        }
    }

    pub fn insert_row(&mut self, table_name: &str, row: Row) {
        self.storage_engine.insert_row(table_name, row);
        self.save_to_file().unwrap();
    }

    pub fn create_table(&mut self, table_name: &str, columns: Vec<String>) {
        self.storage_engine.create_table(table_name, columns);
        self.save_to_file().unwrap();
    }

    fn load_from_file( file_path: &str) -> Result<StorageEngine, std::io::Error>{
        let file = File::open(file_path)?;
        // decryption
        let buffer = Vec::new();
        let storage_engine = StorageEngine::deserialize(&buffer)?;
        Ok(storage_engine)
    }

    fn save_to_file(&self) -> Result<(), std::io::Error>{
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        let mut buffer = Vec::new();
        self.storage_engine.serialize(&mut buffer)?;
        file.write_all(&buffer)?;
        Ok(())
    }
}