### Notes and Exercises on Generic Functions in Rust

#### Introduction to Generic Functions
Generic functions in Rust allow you to write flexible and reusable code by defining functions that can operate on any data type. This is particularly useful when you want to write a function that can handle multiple types without duplicating code.

#### Syntax of Generic Functions
The syntax for defining a generic function in Rust involves using angle brackets (`<>`) to specify the generic type parameter. Here's a basic example:

```rust
fn print_value<T>(value: T) {
    println!("The value is: {:?}", value);
}
```

In this example:
- `T` is a generic type parameter.
- The function `print_value` can accept any type `T` that implements the `Debug` trait (required by the `println!` macro).

#### Use Cases for Generic Functions
1. **Code Reusability**: Write a single function that works with multiple types.
2. **Type Safety**: Ensure that the function operates on the correct types without runtime errors.
3. **Abstraction**: Abstract away the specific types, making the function more general and flexible.

#### Example: Finding the Largest Element
Let's write a generic function to find the largest element in a slice of any type that implements the `PartialOrd` trait.

```rust
fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

- `T: PartialOrd` ensures that the type `T` can be compared using the `>` operator.
- The function returns a reference to the largest element in the slice.

#### Exercises

1. **Swap Function**:
   Write a generic function `swap` that takes two mutable references and swaps their values.

   ```rust
   fn swap<T>(a: &mut T, b: &mut T) {
       // Your code here
   }
   ```

2. **Pair Struct**:
   Define a generic `Pair` struct that holds two values of the same type. Implement a method `swap` that swaps the values of the pair.

   ```rust
   struct Pair<T> {
       first: T,
       second: T,
   }

   impl<T> Pair<T> {
       fn swap(&mut self) {
           // Your code here
       }
   }
   ```

3. **Generic Sum Function**:
   Write a generic function `sum` that takes a slice of numbers (either integers or floats) and returns their sum. Use the `std::ops::Add` trait to ensure the function works with any type that supports addition.

   ```rust
   use std::ops::Add;

   fn sum<T>(numbers: &[T]) -> T
   where
       T: Add<Output = T> + Copy + Default,
   {
       // Your code here
   }
   ```

4. **Generic Display Function**:
   Write a generic function `display` that prints the value of any type that implements the `Display` trait.

   ```rust
   use std::fmt::Display;

   fn display<T: Display>(value: T) {
       // Your code here
   }
   ```

#### Solutions

1. **Swap Function**:
   ```rust
   fn swap<T>(a: &mut T, b: &mut T) {
       std::mem::swap(a, b);
   }
   ```

2. **Pair Struct**:
   ```rust
   struct Pair<T> {
       first: T,
       second: T,
   }

   impl<T> Pair<T> {
       fn swap(&mut self) {
           std::mem::swap(&mut self.first, &mut self.second);
       }
   }
   ```

3. **Generic Sum Function**:
   ```rust
   use std::ops::Add;

   fn sum<T>(numbers: &[T]) -> T
   where
       T: Add<Output = T> + Copy + Default,
   {
       let mut total = T::default();
       for &number in numbers {
           total = total + number;
       }
       total
   }
   ```

4. **Generic Display Function**:
   ```rust
   use std::fmt::Display;

   fn display<T: Display>(value: T) {
       println!("{}", value);
   }
   ```

#### Conclusion
Generic functions in Rust are a powerful tool for writing flexible and reusable code. By understanding how to define and use generic functions, you can write more abstract and type-safe code that works across a wide range of data types. The exercises provided should help you practice and solidify your understanding of generic functions in Rust.