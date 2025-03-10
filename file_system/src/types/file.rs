use crate::traits::filesystem::FileSystemItem;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: usize,
    content: String,
}

// To overload the == and != operators for File, we need to implement the PartialEq trait.
impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}

impl Eq for File {}

impl File {
    pub fn new(name:String,size:usize,content:String) -> Self {
        Self{
            name,
            size,
            content,
    }
}
}

impl FileSystemItem for File {
    fn get_size(&self) -> usize {
        self.size
    }

    fn display(&self) {
        println!("File: {}, (Size: {} bytes)", self.name, self.size);
    }

    fn display_with_indent(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}File: {} (Size: {} bytes)", indent, self.name, self.size);
    }
}
