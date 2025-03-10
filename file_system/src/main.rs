use file_system::{
    traits::filesystem::FileSystemItem,
    types::{file::File, folder::Folder},
};

fn main() {
    let file1 = File::new(
        "file.txt".to_string(),
        1024,
        "Content of file.txt".to_string(),
    );

    let file2 = File::new(
        "file2.txt".to_string(),
        2048,
        "Content of File2.txt".to_string(),
    );

    let mut root_folder = Folder::new("root".to_string());
    root_folder.add_item(Box::new(file1));
    root_folder.add_item(Box::new(file2));

    // Display the total size of the root folder
    println!(
        "Total size of root folder: {} bytes",
        root_folder.get_size()
    );

    // Display the contents of the root folder
    root_folder.display();

    // Using the reference to the vector
    let items_ref = root_folder.get_items();
    for item in items_ref {
        item.display();
    }

    // Using the iterator
    for item in root_folder.get_items() {
        item.display();
    }

    // Using the FolderIterator
    for item in root_folder.iter() {
        item.display();
    }
}
