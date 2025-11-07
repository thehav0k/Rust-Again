//variables and mutability in Rust
fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    // Trying to change the value of x will result in a compile-time error
    // x = 6; // Uncommenting this line will cause an error 
    println!("The value of x after attempting to change is: {}", x);
    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    // Changing the value of y
    y = 15;
    println!("The value of y after change is: {}", y);
}

//why mutability is important?
//Mutability allows variables to be changed after their initial assignment, which is essential for scenarios where
//data needs to be updated or modified during the program's execution. It provides flexibility in managing state and
//enables dynamic behavior in applications.
//However, Rust's default immutability helps prevent unintended side effects and enhances code safety by ensuring
//that variables cannot be altered unless explicitly declared as mutable. This balance between immutability and
//mutability is a key feature of Rust's design, promoting safer and more predictable code.