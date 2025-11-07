# IDE and Plugins

## Recommended: RustRover

### Installation
Download from [jetbrains.com/rust](https://www.jetbrains.com/rust/)

### Features

RustRover is a dedicated Rust IDE by JetBrains with built-in support for:

- **Intelligent Code Completion**: Context-aware suggestions
- **Advanced Debugging**: Powerful debugger with breakpoints and variable inspection
- **Cargo Integration**: Built-in Cargo commands and dependency management
- **Refactoring Tools**: Safe rename, extract method, inline variable, and more
- **Code Analysis**: Real-time error detection and quick fixes
- **Testing Support**: Run and debug tests with visual feedback
- **Version Control**: Integrated Git support

### Quick Setup

1. Download and install RustRover
2. Open your Rust project or create a new one
3. RustRover automatically detects your Rust toolchain
4. Start coding - all features work out of the box!

### Keyboard Shortcuts (macOS)

| Action | Shortcut |
|--------|----------|
| Build | `⌘F9` |
| Run | `⌃R` |
| Debug | `⌃D` |
| Quick Fix | `⌥Enter` |
| Search Everywhere | `Double Shift` |
| Format Code | `⌘⌥L` |

## Alternative IDEs

### Visual Studio Code
- Install `rust-analyzer` extension
- Lightweight and highly customizable
- Free and open source

### IntelliJ IDEA / CLion
- Install Rust plugin from JetBrains marketplace
- Similar features to RustRover

### Neovim/Vim
- Use `rust-analyzer` with LSP configuration
- Lightweight and fast for terminal users

### Zed
- Built-in Rust support
- Modern and fast editor

## RustRover Settings (Optional)

Configure in **Preferences** → **Editor**:

- Enable **Format on Save**: Automatically format code with rustfmt
- Enable **Optimize Imports**: Clean up unused imports
- Configure **Clippy**: Use Clippy as the default checker
- Set **Code Style**: Customize indentation and formatting rules

## Verify Setup

1. Create or open a Rust project in RustRover
2. Open any `.rs` file
3. Check for syntax highlighting
4. Hover over code to see type information and documentation
5. Type to see intelligent code completion
6. Click the run button or use `⌃R` to execute

Your IDE is ready when you see intelligent suggestions, error checking, and can run your code!
