//datatypes in rust
// basic sytax for defining variables with different data types in rust: 
//          let variable_name: data_type = value;
fn main() {
    // Integer types
    let a: i32 = 10; // 32-bit signed integer
    let b: u8 = 255; // 8-bit unsigned integer

    // Floating-point types
    let c: f32 = 3.14; // 32-bit floating point
    let d: f64 = 2.718281828459045; // 64-bit floating point

    // Boolean type
    let e: bool = true;

    // Character type
    let f: char = 'R';

    // Tuple type
    let g: (i32, f64, char) = (42, 6.28, 'A');

    // Array type
    let h: [i32; 5] = [1, 2, 3, 4, 5];
    //string type is not a primitive type in rust, its a collection of characters
    let s: &str = "Hello, Rust!";
    // structure in rust
    struct Point {
        x: f64,
        y: f64,
    }
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    // Print the points
    println!("Point 1: ({}, {})", p1.x, p1.y);
    println!("Point 2: ({}, {})", p2.x, p2.y);

    //enumeration in rust
    enum Direction {
        North,
        South,
        East,
        West,
    }
    let dir: Direction = Direction::North;
    match dir {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East => println!("Heading East"),
        Direction::West => println!("Heading West"),
    }

    // Print the values
    println!("Integer a: {}", a);
    println!("Unsigned Integer b: {}", b);
    println!("Floating-point c: {}", c);
    println!("Floating-point d: {}", d);
    println!("Boolean e: {}", e);
    println!("Character f: {}", f);
    println!("Tuple g: {:?}", g);
    println!("Array h: {:?}", h);
    println!("String s: {}", s);
    //we can directly use the types without annotation as well
    let x = 20; // inferred as i32
    let y = 4.5; // inferred as f64
    // can be printed directly
    println!(f"Inferred Integer x: {x}");

}
// Whats the speciality of rust data types?
//Rust has a rich type system that includes both primitive and complex data types. One of its key features is its emphasis on safety and performance.
//Rust's ownership model, combined with its strict type checking at compile time, helps prevent common programming errors such as null pointer dereferencing and data races in concurrent contexts.
//Additionally, Rust supports user-defined types through structs and enums, allowing developers to create complex data structures tailored to their application's needs.
//Overall, Rust's data types are designed to provide both safety and efficiency, making it a powerful choice for systems programming and beyond.