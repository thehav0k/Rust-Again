// whats ownership and borrowing in Rust?
fn main() {
    // Ownership rules in Rust
    // 1. Each value in Rust has a variable that’s called its owner.
    let s1 = String::from("hello"); // s1 is the owner of the string
    // 2. There can only be one owner at a time.
    let s2 = s1; // ownership of the string is moved from s1 to s2
    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer the owner
    println!("{}", s2); // This works because s2 is now the owner
    // 3. When the owner goes out of scope, the value will be dropped.
    {
        let s3 = String::from("world"); // s3 is the owner of the string
        println!("{}", s3); // s3 is valid here
    } // s3 goes out of scope and the string is dropped here
    // Borrowing in Rust
    let s4 = String::from("hello again");
    let len = calculate_length(&s4); // we pass a reference to s4
    println!("The length of '{}' is {}.", s4, len); // s4 is still valid here
}
// Function that calculates the length of a string slice
fn calculate_length(s: &String) -> usize {
    s.len() // we can use s here because we have a reference to it
}
// Summary of Ownership and Borrowing in Rust
// Ownership is a set of rules that governs how a Rust program manages memory. 
// Each value has a single owner, and when the owner goes out of scope, the value is dropped. This ensures memory safety without needing a garbage collector.
// Borrowing allows you to reference a value without taking ownership of it. 
//This is done using references, which can be either immutable or mutable. 
//Immutable references allow you to read a value without changing it, while mutable references allow you to modify the value.
// Rust enforces rules to ensure that references do not outlive the data they point to, preventing dangling references and data races.
// This ownership and borrowing system is a core feature of Rust that helps ensure memory safety and concurrency without sacrificing performance.
// Difference from cpp and java
// In C++, memory management is manual, requiring developers to allocate and deallocate memory using pointers. 
// This can lead to issues like memory leaks and dangling pointers if not handled carefully.
// Java uses a garbage collector to automatically manage memory, which can introduce overhead and unpredictability in performance.
// Rust's ownership model provides memory safety at compile time without a garbage collector, ensuring efficient and predictable performance.