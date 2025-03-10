use crate::traits::filesystem::FileSystemItem;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    size: usize,
    content: String,
}

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
}
