use std::collections::HashMap;

// Basic Insert, Get, Remove operations on HashMap
pub fn basic_hashmap_operations() {
    let mut scores = HashMap::new();

    // Insert key-value pairs
    scores.insert("Alice", 50);
    scores.insert("Bob", 80);

    // Retrieve value
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice's score not found"),
    }

    // Remove a key-value pair
    scores.remove("Bob");

    println!("After removing Bob: {:?}", scores);
}
