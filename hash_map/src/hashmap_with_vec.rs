use std::collections::HashMap;

pub fn hashmap_with_vec() {
    let mut student_scores: HashMap<&str, Vec<i32>> = HashMap::new();

    // Insert multiple scores for "Alice"
    student_scores.entry("Alice").or_insert(vec![]).push(85);
    student_scores.entry("Alice").or_insert(vec![]).push(90);
    student_scores.entry("Bob").or_insert(vec![]).push(88);

    println!("{:?}", student_scores);
}
