# Build and Run

## Basic Commands

### Build Your Project
```bash
# Debug build (faster compilation, slower execution)
cargo build

# Release build (optimized, slower compilation, faster execution)
cargo build --release
```

### Run Your Project
```bash
# Build and run in one command (debug mode)
cargo run

# Run with release optimizations
cargo run --release

# Pass arguments to your program
cargo run -- arg1 arg2
```

### Check Without Building
```bash
# Fast syntax and type checking (no binary output)
cargo check
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name::
```

## Code Quality

### Format Code
```bash
# Format all Rust files
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

### Lint Code
```bash
# Run Clippy linter
cargo clippy

# Treat warnings as errors
cargo clippy -- -D warnings
```

## Cleaning Build Artifacts

```bash
# Remove target directory and build artifacts
cargo clean
```

## Running Examples

```bash
# Run a specific example from examples/ directory
cargo run --example example_name
```

## Build Profiles

Cargo uses different optimization levels:

| Profile | Command | Optimization | Debug Info | Use Case |
|---------|---------|--------------|------------|----------|
| Dev     | `cargo build` | Low (fast compile) | Yes | Development |
| Release | `cargo build --release` | High (slow compile) | No | Production |

### Custom Profile (Cargo.toml)
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
```

## Watch Mode (Auto-rebuild)

Install cargo-watch:
```bash
cargo install cargo-watch
```

Use it:
```bash
# Automatically rebuild on file changes
cargo watch -x build

# Auto-run on changes
cargo watch -x run

# Auto-test on changes
cargo watch -x test
```

## Documentation

```bash
# Generate and open documentation
cargo doc --open

# Include private items
cargo doc --document-private-items
```

## Common Workflow

```bash
# 1. Check code compiles (fast)
cargo check

# 2. Run tests
cargo test

# 3. Lint code
cargo clippy

# 4. Format code
cargo fmt

# 5. Run the program
cargo run
```

## Quick Reference

```bash
cargo new project    # Create new project
cargo build          # Compile project
cargo run            # Build and run
cargo test           # Run tests
cargo check          # Fast compile check
cargo fmt            # Format code
cargo clippy         # Lint code
cargo clean          # Clean build files
cargo update         # Update dependencies
```

Happy coding!
