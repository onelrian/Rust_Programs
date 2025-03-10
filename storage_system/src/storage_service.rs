use std::collections::HashMap;

use crate::storage::StorageItem;

pub struct StorageService<T> {
    items: HashMap<i32, StorageItem<T>>,
}

impl<T: std::fmt::Debug> StorageService<T> 
    {
    pub fn new() -> Self {
        StorageService {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: StorageItem<T>) {
        self.items.insert(item.get_id(), item);
    }

    pub fn remove_item(&mut self, id: i32) -> Option<StorageItem<T>> {
        self.items.remove(&id)
    }

    pub fn get_item(&self, id: i32) -> Option<&StorageItem<T>> {
        self.items.get(&id)
    }

    pub fn display_all_items(&self) {
        for (id, item) in &self.items {
            println!("ID: {}, Value: {:?}", id, item.get_value());
        }
    }
}