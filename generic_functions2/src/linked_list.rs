struct Node<T: PartialOrd + Copy> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + Copy> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }

    pub fn insert(&self, data: T) -> Self {
        if data < self.data {
            self = &Node {
                data,
                next: Some(Box::new(*self)),
            };
        }
        self
    }
}
