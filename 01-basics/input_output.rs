// taking input of different types from user and displaying them back
use std::io;
fn main(){
    let mut input_string = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let input_string = input_string.trim();
    println!("You entered the string: {}", input_string);

    let mut input_int = String::new();
    println!("Enter an integer:");
    io::stdin().read_line(&mut input_int).expect("Failed to read line");
    let input_int: i32 = input_int.trim().parse().expect("Please enter a valid integer");
    println!("You entered the integer: {}", input_int);

    let mut input_float = String::new();
    println!("Enter a float:");
    io::stdin().read_line(&mut input_float).expect("Failed to read line");
    let input_float: f64 = input_float.trim().parse().expect("Please enter a valid float");
    println!("You entered the float: {}", input_float);
    
    //Taking input of array of integers
    let mut input_array = String::new();
    println!("Enter integers separated by spaces:");
    io::stdin().read_line(&mut input_array).expect("Failed to read line");
    let input_array: Vec<i32> = input_array
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect(); // Collecting into a vector of integers
    println!("You entered the array of integers: {:?}", input_array);
}
// breakdown of the input process:
// 1. We create a mutable String variable to hold the user input.
// 2. We prompt the user for input using println!.
// 3. We read the input from standard input (stdin) using io::stdin().read_line().
// 4. We trim any whitespace or newline characters from the input using trim().
// 5. For non-string inputs, we parse the trimmed input into the desired type (i32 for integer, f64 for float) using parse().
// 6. We use expect() to handle any potential errors during parsing, providing a user-friendly error message.
// 7. Finally, we display the entered input back to the user using println!.

// Why Rust's type system is beneficial for handling user input?
// Rust's type system is beneficial for handling user input because it enforces strict type checking at compile time,
// reducing the likelihood of runtime errors. When parsing user input, Rust requires explicit conversion to the desired type,
// which helps catch potential issues early in the development process.
// This ensures that the data being processed is of the expected type, leading to more robust and reliable code.
// Additionally, Rust's use of the Result type for error handling allows developers to gracefully manage parsing errors, 
// providing clear feedback to users when their input is invalid. Overall, 
// Rust's type system enhances code safety and maintainability when dealing with user input.