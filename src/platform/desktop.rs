#![cfg(feature = "desktop")]

use crate::Todos;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{env, fs::File, path::PathBuf};
use tracing::info;

use super::Store;

const TODO_FILE: &str = "todo.json";

pub struct FileStore {
    path: PathBuf,
}

impl Default for FileStore {
    fn default() -> Self {
        let path = env::current_dir().unwrap().join(TODO_FILE);
        info!("desktop FileStore path: {:?}", path);
        Self { path }
    }
}

impl Store for FileStore {
    fn get(&self) -> crate::Todos {
        if let Ok(mut file) = File::open(&self.path) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            if content.is_empty() {
                return Todos {
                    items: HashMap::new(),
                    next_id: 0,
                };
            }
            serde_json::from_str(&content).unwrap()
        } else {
            File::create(&self.path).unwrap();
            return Todos {
                items: HashMap::new(),
                next_id: 0,
            };
        }
    }

    fn set(&self, item: &crate::Todos) {
        let content = serde_json::to_string(&item).unwrap();
        let mut file = File::create(&self.path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}

pub fn get_store() -> impl Store {
    FileStore::default()
}
