pub struct StorageItem<T> {
    id: i32,
    value: T,
}

impl<T> StorageItem<T> {
    pub fn new(id: i32, value: T) -> Self {
        StorageItem { id, value }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}