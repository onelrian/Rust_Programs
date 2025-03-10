use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct User {
    pub name: String,
}

pub fn hashmap_with_structs() {
    let mut user_scores: HashMap<User, i32> = HashMap::new();

    let user1 = User { name: "Alice".to_string() };
    let user2 = User { name: "Bob".to_string() };

    // Insert user scores into the hashmap
    user_scores.insert(user1, 90);
    user_scores.insert(user2, 80);

    println!("User Scores with Structs: {:?}", user_scores);
}
