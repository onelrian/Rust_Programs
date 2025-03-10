use std::collections::{HashMap, HashSet};

// HashMap with HashSet to store unique tags for users
pub fn hashmap_with_hashset() {
    let mut user_tags: HashMap<&str, HashSet<&str>> = HashMap::new();

    // Add tags to "Alice"
    user_tags.entry("Alice").or_insert(HashSet::new()).insert("rust");
    user_tags.entry("Alice").or_insert(HashSet::new()).insert("developer");

    // Add tags to "Bob"
    user_tags.entry("Bob").or_insert(HashSet::new()).insert("engineer");

    println!("HashMap with HashSet: {:?}", user_tags);
}
