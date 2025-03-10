

Here are some code examples that demonstrate the use of hash maps in Rust:

**Example 1: Caching**

```rust
use std::collections::HashMap;

fn main() {
    let mut cache = HashMap::new();

    // Add some data to the cache
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");

    // Retrieve data from the cache
    println!("Value for key1: {}", cache.get("key1").unwrap());
    println!("Value for key2: {}", cache.get("key2").unwrap());

    // Update data in the cache
    cache.insert("key1", "new_value1");

    // Retrieve updated data from the cache
    println!("Updated value for key1: {}", cache.get("key1").unwrap());
}
```

**Example 2: Configuration storage**

```rust
use std::collections::HashMap;

fn main() {
    let mut config = HashMap::new();

    // Add some configuration data
    config.insert("database_url", "localhost:5432");
    config.insert("username", "admin");
    config.insert("password", "password");

    // Retrieve configuration data
    println!("Database URL: {}", config.get("database_url").unwrap());
    println!("Username: {}", config.get("username").unwrap());
    println!("Password: {}", config.get("password").unwrap());
}
```

**Example 3: Data aggregation**

```rust
use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();

    // Add some data to the hash map
    data.insert("key1", 10);
    data.insert("key2", 20);
    data.insert("key3", 30);

    // Aggregate the data
    let sum: i32 = data.values().sum();

    println!("Sum of values: {}", sum);
}
```

**Example 4: Data normalization**

```rust
use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();

    // Add some data to the hash map
    data.insert("key1", "value1");
    data.insert("key2", "value2");
    data.insert("key3", "value3");

    // Normalize the data
    let normalized_data: HashMap<String, String> = data.iter().map(|(k, v)| (k.to_lowercase(), v.to_lowercase())).collect();

    println!("Normalized data: {:?}", normalized_data);
}
```

**Example 5: Graph algorithms**

```rust
use std::collections::HashMap;

fn main() {
    let mut graph = HashMap::new();

    // Add some nodes to the graph
    graph.insert("A", vec!["B", "C"]);
    graph.insert("B", vec!["A", "D"]);
    graph.insert("C", vec!["A", "D"]);
    graph.insert("D", vec!["B", "C"]);

    // Find the shortest path between two nodes
    let shortest_path = find_shortest_path(&graph, "A", "D");

    println!("Shortest path: {:?}", shortest_path);
}

fn find_shortest_path(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> Vec<String> {
    // Implement a graph algorithm to find the shortest path
    // ...
}
```

These are just a few examples of how hash maps can be used in Rust. Hash maps are a versatile data structure that can be used in many different contexts, and are an essential part of any Rust programmer's toolkit.