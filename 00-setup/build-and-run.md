# Build and Run

## Quick Reference

```bash
# Project Setup
cargo new project         # Create new project
cargo init               # Initialize in existing directory

# Development
cargo check              # Fast compile check (no binary)
cargo build              # Build debug version
cargo run                # Build and run
cargo run -- arg1 arg2   # Run with arguments

# Testing & Quality
cargo test               # Run tests
cargo fmt                # Format code
cargo clippy             # Lint code

# Production
cargo build --release    # Build optimized version
cargo run --release      # Run optimized version

# Maintenance
cargo clean              # Remove build artifacts
cargo update             # Update dependencies
cargo doc --open         # Generate and view docs
```

---

## 1. Terminal Workflow (No IDE)

### Text Editors

#### nano (Beginner-Friendly)

```bash
nano hello.rs            # Create/edit file
# Ctrl+O to save, Ctrl+X to exit
```

**Shortcuts:** `Ctrl+O` (save) • `Ctrl+X` (exit) • `Ctrl+K` (cut) • `Ctrl+U` (paste) • `Ctrl+W` (search)

#### vim (Advanced)

```bash
vim hello.rs             # Create/edit file
# Press 'i' for insert mode, 'Esc' to exit, ':wq' to save and quit
```

**Commands:** `i` (insert) • `Esc` (normal mode) • `:w` (save) • `:q` (quit) • `:wq` (save & quit) • `:q!` (quit without save)

### Compile with rustc (Single Files)

```bash
# Create file
nano hello.rs
# Type: fn main() { println!("Hello, Rust!"); }

# Compile and run
rustc hello.rs
./hello

# With optimizations
rustc -O hello.rs
./hello

# Custom output name
rustc hello.rs -o my_program
./my_program
```

**When to use:** Learning basics, single-file programs, quick experiments

---

## 2. Cargo Workflow (Projects)

### Create and Run

```bash
# Create new project
cargo new my_project
cd my_project

# Build and run (one command)
cargo run

# Or separate steps
cargo build
./target/debug/my_project
```

**When to use:** Multi-file projects, dependencies, team collaboration

### Build Modes

| Mode | Command | Speed | Performance | Use |
|------|---------|-------|-------------|-----|
| Debug | `cargo build` | Fast compile | Slower runtime | Development |
| Release | `cargo build --release` | Slow compile | Fast runtime | Production |

### Quick Check (No Binary)

```bash
cargo check              # Fast - only checks if code compiles
```

---

## 3. Testing

```bash
# Run all tests
cargo test

# Run with output visible
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in module
cargo test module_name::
```

---

## 4. Code Quality

### Format

```bash
cargo fmt                    # Format all files
cargo fmt -- --check         # Check without modifying
```

### Lint

```bash
cargo clippy                 # Run linter
cargo clippy -- -D warnings  # Treat warnings as errors
```

---

## 5. Advanced Features

### Custom Build Profiles

Add to `Cargo.toml`:

```toml
[profile.dev]
opt-level = 0               # No optimization (fast compile)

[profile.release]
opt-level = 3               # Maximum optimization
lto = true                  # Link-time optimization
codegen-units = 1           # Better optimization
```

### Auto-Rebuild on Changes

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch and rebuild
cargo watch -x check        # Check on changes
cargo watch -x test         # Test on changes
cargo watch -x run          # Run on changes
```

### Documentation

```bash
cargo doc --open                      # Generate and open docs
cargo doc --document-private-items    # Include private items
```

### Running Examples

```bash
cargo run --example example_name
```

---

## 6. Development Workflow

**Recommended sequence:**

```bash
1. cargo check          # Fast compile check
2. cargo test           # Run tests
3. cargo clippy         # Lint code
4. cargo fmt            # Format code
5. cargo run            # Run program
```

---

## 7. Cleanup

```bash
cargo clean             # Remove target/ directory
```

---

## Comparison: rustc vs cargo

| Feature | rustc | cargo |
|---------|-------|-------|
| **Use Case** | Single files, learning | Real projects |
| **Dependencies** | Manual | Automatic |
| **Build Tool** | None | Built-in |
| **Project Structure** | Flat | Organized |
| **Best For** | Quick tests | Production code |

Happy coding! 🦀
