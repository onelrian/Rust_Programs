use file_system::{traits::filesystem::FileSystemItem, tree::display_tree, types::{file::File, folder::Folder}};


fn main() {
    // Create files
    let file1 = File::new("file1.txt".to_string(),1024,"Content of File1".to_string());

    let file2 = File::new("file2.txt".to_string(),2048,"This is the content of file2.txt".to_string());

    // Create a subfolder
    let mut subfolder = Folder::new("subfolder".to_string());
    subfolder.add_item(Box::new(file2));

    // Create the root folder and add items
    let mut root_folder = Folder::new("root".to_string());
    root_folder.add_item(Box::new(file1));
    root_folder.add_item(Box::new(subfolder));

    // Display the total size of the root folder
    println!("Total size of root folder: {} bytes", root_folder.get_size());

    // Display the contents of the root folder
    root_folder.display();

    // Display the folder structure with indentation
    display_tree(&root_folder, 0);

    // Demonstrate file comparison
    let file1 = File::new("file1.txt".to_string(),1024,"This is the content of file1.txt".to_string());
    let file1_copy = File::new("file1.txt".to_string(),1024,"This is the content of file1.txt".to_string());

    if file1 == file1_copy {
        println!("Files are identical");
    } else {
        println!("Files are different");
    }

    
}