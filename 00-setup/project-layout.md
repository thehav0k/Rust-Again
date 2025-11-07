# Project Layout

## Creating a New Project

```bash
# Create a binary (executable) project
cargo new project_name

# Create a library project
cargo new --lib library_name
```

## Standard Project Structure

```
project_name/
├── Cargo.toml          # Project manifest and dependencies
├── Cargo.lock          # Locked dependency versions (auto-generated)
├── src/
│   └── main.rs         # Entry point for binary
│   └── lib.rs          # Entry point for library
├── tests/              # Integration tests
│   └── integration_test.rs
├── benches/            # Benchmarks
│   └── benchmark.rs
└── examples/           # Example code
    └── example.rs
```

## Cargo.toml Explained

```toml
[package]
name = "project_name"
version = "0.1.0"
edition = "2021"           # Rust edition

[dependencies]
# Add external crates here
# serde = "1.0"

[dev-dependencies]
# Test-only dependencies
# criterion = "0.5"
```

## Source Code Organization

### Binary Project (main.rs)
```rust
fn main() {
    println!("Hello, world!");
}
```

### Library Project (lib.rs)
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

## Module Organization

For larger projects:

```
src/
├── main.rs             # Entry point
├── lib.rs              # Library root
├── utils.rs            # Utility module
└── models/             # Submodule directory
    ├── mod.rs          # Module declaration
    ├── user.rs
    └── post.rs
```

## Best Practices

1. **Keep `main.rs` minimal** - Move logic to library modules
2. **Use modules** - Organize code into logical units
3. **Write tests** - Keep tests close to implementation
4. **Document public APIs** - Use doc comments (`///`)
5. **Follow naming conventions**:
   - Files: `snake_case.rs`
   - Functions: `snake_case`
   - Types: `PascalCase`
   - Constants: `SCREAMING_SNAKE_CASE`

## Project Templates

```bash
# Create with a specific template
cargo new my_app
cd my_app

# Add common dependencies
cargo add serde --features derive
cargo add tokio --features full
```

Your project structure should be clean, logical, and follow Rust conventions!
