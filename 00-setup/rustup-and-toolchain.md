# Rustup and Toolchain

## Installing Rust

### macOS/Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows
Download and run [rustup-init.exe](https://rustup.rs/)

## Verify Installation

```bash
rustc --version
cargo --version
rustup --version
```

## Essential Toolchain Components

### Rust Compiler and Cargo
- `rustc` - The Rust compiler
- `cargo` - Package manager and build tool

### Install Additional Components

```bash
# Rust formatter
rustup component add rustfmt

# Rust linter
rustup component add clippy

# Rust Language Server (for IDE support)
rustup component add rust-analyzer
```

## Managing Toolchains

```bash
# Update Rust
rustup update

# Show installed toolchains
rustup show

# Set default toolchain
rustup default stable

# Install nightly toolchain (optional)
rustup install nightly
```

## Quick Verification

```bash
# Format check
cargo fmt --version

# Linter check
cargo clippy --version

# Build tool check
cargo --version
```

## Next Steps

Once installed, you're ready to create your first Rust project with `cargo new project_name`.
