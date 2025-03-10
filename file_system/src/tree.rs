use crate::{traits::filesystem::FileSystemItem, types::folder::Folder};

fn display_tree(folder: &Folder, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}Folder: {}", indent, folder.get_name());

    for item in folder.get_items() {
        match item.as_ref() {
            FileSystemItem::File(file) => {
                println!("{}  File: {} (Size: {} bytes)", indent, file.name, file.size);
            }
            FileSystemItem::Folder(subfolder) => {
                display_tree(subfolder, depth + 1);
            }
        }
    }
}