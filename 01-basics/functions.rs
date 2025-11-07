/*
functions syntax in rust: fn function_name(parameters) -> return_type {
    // function body
    // optional return statement
}
    we dont need to speciify return in function body, the last expression is returned by default
    if there is no return value, we can use () as return type or omit it entirely
*/

// Function that adds two integers and returns the sum
fn add(a: i32, b: i32) -> i32 {
    a + b // last expression is returned
}
// Function that greets a user by name and returns a greeting message
fn greet(name: &str) -> String {
    format!("Hello, {}!", name) // last expression is returned
}
// A recursive function that calculates the factorial of a number
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
// higher order functions
fn higher_order_example<F>(func: F) -> i32
where
    F: Fn(i32) -> i32,
{
    func(10) // call the passed function with argument 10
}
// 
fn afunction() -> i32 {
    let x ={
        let y = 5;
        y + 2 // this expression is returned from the block
    };
    x // return x as the last expression so the function returns i32
}

// Main function to demonstrate the usage of the above functions
fn main() {
    let result = add(5, 3);
    println!("The sum of 5 and 3 is: {}", result);

    let greeting = greet("Alice");
    println!("{}", greeting);
    let fact = factorial(5);
    println!("The factorial of 5 is: {}", fact);
    let square = |x: i32| x * x; // closure that squares a number
    let ho_result = higher_order_example(square);
    println!("The result of the higher-order function is: {}", ho_result);
    let afun_result = afunction();
    println!("The result of afunction is: {}", afun_result);
}

// special note on functions in rust
// Functions in Rust are defined using the `fn` keyword, followed by the function name, parameters in parentheses, and an optional return type.
// Rust functions can return values without an explicit `return` statement; the last expression in the function body is returned by default.
// Functions can take parameters of various types, and Rust's strong type system ensures that the types of arguments match the function's parameter types.
// Rust supports recursion, allowing functions to call themselves, which is useful for problems like calculating factorials or traversing data structures.
// Functions help in organizing code into reusable blocks, improving readability and maintainability.
// Rust also supports higher-order functions, allowing functions to take other functions as parameters or return them.
// Overall, functions are a fundamental building block in Rust programming, enabling modular and efficient code design.