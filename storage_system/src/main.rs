mod storage;
mod storage_service;

use storage::StorageItem;
use storage_service::StorageService;

fn main() {
    let mut storage = StorageService::new();

    let item1 = StorageItem::new(1, "Hello");
    let item2 = StorageItem::new(2, 42);

    storage.add_item(item1);
    storage.add_item(item2);

    storage.display_all_items();

    let item = storage.get_item(1).unwrap();
    println!("Item 1 value: {}", item.get_value());

    storage.remove_item(2);

    storage.display_all_items();
}