// control flows in rust
fn main() {
// if-else statement
let number = 7;
if number < 5 {
println!("The number is less than 5");
} else if number == 5 {
println!("The number is equal to 5");
} else {
println!("The number is greater than 5");
}
// switch statement( match in rust)
let day = 3;
match day {
1 => println!("Monday"),
2 => println!("Tuesday"),
3 => println!("Wednesday"),
4 => println!("Thursday"),
5 => println!("Friday"),
6 => println!("Saturday"),
7 => println!("Sunday"),
_ => println!("Invalid day"), //default case
}

// loop statement( infinite loop with break)
let mut count = 0;
loop {
count += 1;
if count == 3 {
println!("Reached count of 3, breaking the loop");
break;
}
}

// while loop
let mut n = 1;
while n <= 5 {
println!("n is: {}", n);
n += 1;
}

// different ways to use for loop
// iterating over an array
let arr = [10, 20, 30, 40, 50];
for element in arr.iter() {
println!("Array element: {}", element);
}
// iterating over a range
for i in 1..=5 {
println!("Range value: {}", i);
}
// iterating with enumerate to get index and value
for (index, value) in arr.iter().enumerate() {
println!("Index: {}, Value: {}", index, value);
}
}
// special note on control flow in rust
// In Rust, control flow constructs like if-else, match, loops (loop, while, for) are used to dictate the execution path of a program based on conditions and iterations.
// Rust emphasizes safety and explicitness in its control flow mechanisms. For instance, the match statement is exhaustive, meaning all possible cases must be handled, which helps prevent runtime errors.
// Additionally, Rust's ownership model interacts with control flow, ensuring that variables are used safely within their scopes, especially in loops and conditional branches. This design promotes writing robust and error-free code.