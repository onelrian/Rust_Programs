pub struct GenericList<T> {
    elements: Vec<T>,
    capacity: usize,
}

impl<T> GenericList<T> {
   pub fn isFull(&self) -> bool {
        self.elements.len() == self.capacity
    }
}
impl<T: PartialEq> GenericList<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if self.isFull() {
            println!("List is full. Cannot insert.");
            return;
        }
        if index <= self.elements.len() {
            self.elements.insert(index, value);
        } else {
            println!("Index out of bounds.");
        }
    }

    pub fn remove(&mut self, value: &T) {
        if let Some(pos) = self.elements.iter().position(|x| x == value) {
            self.elements.remove(pos);
        } else {
            println!("Value not found.");
        }
    }
}