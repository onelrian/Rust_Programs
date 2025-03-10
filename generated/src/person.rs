pub struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name:String,age:u32) -> Self{
        Self{
            name,
            age
        }
    }

    pub fn introduce(&self) {
        println!("Hey My Name is {} and I am {} Old", self.name,&self.age);
    } 
}