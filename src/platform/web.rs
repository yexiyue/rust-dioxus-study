#![cfg(feature="web")]

use std::ops::Deref;
use web_sys::Storage;
use super::Store;

const TODOS_KEY: &str = "todos";

pub struct LocalStorage(Storage);

impl Deref for LocalStorage{
    type Target=Storage;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}


impl Default for LocalStorage {
    fn default() -> Self {
        let window = web_sys::window().unwrap();
        Self(
            window
                .local_storage()
                .unwrap()
                .expect("user did not allow local storage"),
        )
    }
}

impl Store for LocalStorage {
    fn get(&self)->crate::Todos {
        if let Ok(Some(value)) = self.0.get(TODOS_KEY) {
            serde_json::from_str(&value).unwrap()
        }else{
            Default::default()
        }
    }

    fn set(&self,item:&crate::Todos) {
        let content=serde_json::to_string(&item).unwrap();
        self.0.set_item(TODOS_KEY, &content).unwrap();
    }
}



pub fn get_store() -> impl Store {
    LocalStorage::default()
}

