use hash_map::hashmap_with_structs;

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic_hashmap_operations() {
        // We can test insert, remove, and get operations here
        let mut scores = std::collections::HashMap::new();
        scores.insert("Alice", 50);
        scores.insert("Bob", 80);
        
        // Check if Alice's score is 50
        assert_eq!(scores.get("Alice"), Some(&50));
        
        // Remove Bob and check if Bob's score is removed
        scores.remove("Bob");
        assert_eq!(scores.get("Bob"), None);
    }

    #[test]
    fn test_nested_hashmap() {
        let mut company: std::collections::HashMap<&str, std::collections::HashMap<&str, i32>> = std::collections::HashMap::new();
        company.entry("Engineering").or_insert(std::collections::HashMap::new()).insert("Alice", 70000);
        company.entry("HR").or_insert(std::collections::HashMap::new()).insert("Bob", 60000);

        // Check that Alice's salary is in the Engineering department
        assert_eq!(company.get("Engineering").unwrap().get("Alice"), Some(&70000));

        // Check that Bob's salary is in the HR department
        assert_eq!(company.get("HR").unwrap().get("Bob"), Some(&60000));
    }

    #[test]
    fn test_hashmap_with_structs() {
        // Define structs and test insertion into hashmap
        use crate::hashmap_with_structs::User;
        let mut user_scores = std::collections::HashMap::new();

        let user1 = User { name: "Alice".to_string() };
        let user2 = User { name: "Bob".to_string() };

        user_scores.insert(user1, 90);
        user_scores.insert(user2, 80);

        // Test that the users and their scores are correctly inserted
        assert_eq!(user_scores.len(), 2);
    }

    #[test]
    fn test_hashmap_with_hashset() {
        // Define HashMap with HashSet and test insertion of unique tags
        let mut user_tags: std::collections::HashMap<&str, std::collections::HashSet<&str>> = std::collections::HashMap::new();

        user_tags.entry("Alice").or_insert(std::collections::HashSet::new()).insert("rust");
        user_tags.entry("Alice").or_insert(std::collections::HashSet::new()).insert("developer");
        user_tags.entry("Bob").or_insert(std::collections::HashSet::new()).insert("engineer");

        // Test that Alice has both 'rust' and 'developer' tags
        let alice_tags = user_tags.get("Alice").unwrap();
        assert!(alice_tags.contains("rust"));
        assert!(alice_tags.contains("developer"));

        // Test that Bob has 'engineer' tag
        let bob_tags = user_tags.get("Bob").unwrap();
        assert!(bob_tags.contains("engineer"));
    }

        // Test for HashMap with Vec as values
        #[test]
        fn test_hashmap_with_vec() {
            let mut student_scores: std::collections::HashMap<&str, std::vec::Vec<i32>> = std::collections::HashMap::new();
    
            // Insert scores for Alice and Bob
            student_scores.entry("Alice").or_insert(vec![]).push(85);
            student_scores.entry("Alice").or_insert(vec![]).push(90);
            student_scores.entry("Bob").or_insert(vec![]).push(88);
    
            // Assert that Alice has two scores: 85 and 90
            assert_eq!(student_scores.get("Alice").unwrap(), &vec![85, 90]);
    
            // Assert that Bob has one score: 88
            assert_eq!(student_scores.get("Bob").unwrap(), &vec![88]);
        }
}
