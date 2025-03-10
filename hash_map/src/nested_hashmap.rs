use std::collections::HashMap;

// Example of a Nested HashMap
pub fn nested_hashmap() {
    let mut company: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    // Insert a salary for Alice in Engineering
    company.entry("Engineering").or_insert(HashMap::new()).insert("Alice", 70000);
    
    // Insert a salary for Bob in HR
    company.entry("HR").or_insert(HashMap::new()).insert("Bob", 60000);

    println!("Nested HashMap: {:?}", company);
}
