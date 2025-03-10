use crate::types::folder::Folder;

pub fn display_tree(folder: &Folder, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}Folder: {}", indent, folder.get_name());

    for item in folder.get_items() {
        item.display_with_indent(depth + 1);
    }
}