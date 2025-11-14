# Detailed Setup Guide

Complete setup instructions with troubleshooting for all platforms.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Platform-Specific Setup](#platform-specific-setup)
- [Installation Steps](#installation-steps)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)
- [Advanced Configuration](#advanced-configuration)

## Prerequisites

### Required Software

1. **Python 3.8 or higher**
   ```bash
   python --version  # Should show 3.8+
   ```

2. **Rust 1.70 or higher**
   ```bash
   rustc --version  # Should show 1.70+
   cargo --version
   ```

3. **pip** (Python package manager)
   ```bash
   pip --version
   ```

### Optional but Recommended

- **Virtual environment** (venv or conda)
- **Make** (for convenient commands)
- **Git** (for version control)

## Platform-Specific Setup

### Linux (Ubuntu/Debian)

```bash
# Update package list
sudo apt update

# Install Python development headers
sudo apt install python3-dev python3-pip

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build essentials
sudo apt install build-essential pkg-config libssl-dev

# Verify installations
python3 --version
rustc --version
```

### macOS

```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Python (if not using system Python)
brew install python@3.11

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installations
python3 --version
rustc --version
```

### Windows

1. **Install Python**
   - Download from [python.org](https://www.python.org/downloads/)
   - During installation, check "Add Python to PATH"
   - Verify: `python --version` in Command Prompt

2. **Install Rust**
   - Download from [rustup.rs](https://rustup.rs/)
   - Run `rustup-init.exe`
   - Follow prompts (default installation is fine)
   - Verify: `rustc --version` in Command Prompt

3. **Install Visual Studio Build Tools**
   - Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
   - Install "Desktop development with C++"
   - Required for compiling Rust code

## Installation Steps

### Step 1: Clone or Download

```bash
# If using Git
git clone <repository-url>
cd rust-python-integration

# Or download and extract the ZIP file
```

### Step 2: Create Virtual Environment (Recommended)

```bash
# Using venv
python -m venv venv

# Activate on Linux/macOS
source venv/bin/activate

# Activate on Windows
venv\Scripts\activate

# Your prompt should now show (venv)
```

### Step 3: Install Python Dependencies

```bash
make install
```

Or manually:
```bash
pip install -r requirements.txt
```

This installs:
- `maturin>=1.0` - Builds Rust extensions
- `pytest>=7.0` - Testing framework
- `black>=22.0` - Code formatter
- `mypy>=0.990` - Type checker

### Step 4: Build Rust Module

**Debug build (fast compilation, slower runtime):**
```bash
make build
```

**Release build (slow compilation, fast runtime):**
```bash
make build-release
```

Manual build:
```bash
cd rust_processor
maturin develop                    # Debug
maturin develop --release          # Release
cd ..
```

**Expected output:**
```
🔗 Found pyo3 bindings
🐍 Found CPython 3.11 at /usr/bin/python3
   Compiling rust_processor v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 45.32s
📦 Built wheel to target/wheels/rust_processor-0.1.0-*.whl
✏️  Setting installed package as editable
🛠 Installed rust_processor-0.1.0
```

### Step 5: Verify Installation

```bash
python -c "import rust_processor; print('✅ Rust module loaded successfully!')"
```

Should output: `✅ Rust module loaded successfully!`

### Step 6: Run Examples

```bash
make example
```

Or manually:
```bash
python examples/basic_usage.py
```

## Verification Checklist

Run through this checklist to ensure everything is working:

- [ ] Python version 3.8+ (`python --version`)
- [ ] Rust installed (`rustc --version`)
- [ ] Dependencies installed (`pip list | grep maturin`)
- [ ] Rust module builds without errors (`make build`)
- [ ] Can import module (`python -c "import rust_processor"`)
- [ ] Examples run successfully (`make example`)
- [ ] Benchmarks work (`make benchmark`)

## Troubleshooting

### Common Issues

#### 1. ImportError: No module named 'rust_processor'

**Symptoms:**
```python
ImportError: No module named 'rust_processor'
```

**Causes & Solutions:**

a) **Module not built yet**
   ```bash
   make build
   ```

b) **Wrong Python interpreter**
   ```bash
   # Make sure you're using the same Python that built the module
   which python
   python -c "import sys; print(sys.executable)"
   ```

c) **Virtual environment not activated**
   ```bash
   source venv/bin/activate  # Linux/macOS
   venv\Scripts\activate     # Windows
   ```

#### 2. maturin: command not found

**Solution:**
```bash
pip install maturin
```

If still not found:
```bash
# Add to PATH (Linux/macOS)
export PATH="$HOME/.local/bin:$PATH"

# Or install in user directory
pip install --user maturin
```

#### 3. cargo: command not found

**Solution:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH
source $HOME/.cargo/env

# Verify
cargo --version
```

#### 4. Build fails with "linker 'cc' not found"

**Linux:**
```bash
sudo apt install build-essential
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
Install Visual Studio Build Tools (see Platform-Specific Setup)

#### 5. Build fails with "error: failed to run custom build command"

**Full error might include:** "could not find native static library"

**Solution:**
```bash
# Linux
sudo apt install libssl-dev pkg-config

# macOS
brew install openssl pkg-config

# Set environment variables if needed
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

#### 6. Performance not as expected

**Issue:** Rust is only 2-3x faster instead of 20-50x

**Causes & Solutions:**

a) **Debug build instead of release**
   ```bash
   make build-release  # Much faster!
   ```

b) **Small dataset**
   ```bash
   # Speedup increases with dataset size
   make benchmark-full  # Tests larger datasets
   ```

c) **CPU throttling**
   - Close other applications
   - Check CPU temperature
   - Ensure power mode is "Performance" (laptops)

#### 7. Module built but functions missing

**Symptoms:**
```python
AttributeError: module 'rust_processor' has no attribute 'compute_stats'
```

**Solution:**
```bash
# Clean and rebuild
make clean
make build
```

#### 8. "PYO3_PYTHON environment variable is not set"

**Solution:**
```bash
export PYO3_PYTHON=$(which python3)
make build
```

#### 9. Tests fail with encoding errors

**Solution:**
```bash
# Set UTF-8 encoding (Linux/macOS)
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Windows (PowerShell)
$env:PYTHONIOENCODING="utf-8"
```

### Platform-Specific Issues

#### macOS: "ld: library not found for -lssl"

```bash
brew install openssl
export LDFLAGS="-L/usr/local/opt/openssl/lib"
export CPPFLAGS="-I/usr/local/opt/openssl/include"
make build
```

#### Windows: "error: linker link.exe not found"

Install Visual Studio Build Tools with C++ development tools.

#### Linux: "error: failed to compile rust_processor"

```bash
# Install all required development packages
sudo apt install build-essential libssl-dev pkg-config python3-dev
```

## Advanced Configuration

### Using Different Python Versions

```bash
# Build for specific Python version
python3.11 -m venv venv311
source venv311/bin/activate
pip install maturin
cd rust_processor
maturin develop
```

### Cross-Compilation

Build for different platforms:

```bash
# Install cross-compilation target
rustup target add x86_64-unknown-linux-musl

# Build
cd rust_processor
maturin build --release --target x86_64-unknown-linux-musl
```

### Environment Variables

Useful environment variables:

```bash
# Force specific Python
export PYO3_PYTHON=/usr/bin/python3.11

# Set optimization level
export RUSTFLAGS="-C target-cpu=native"

# Enable debug output
export RUST_LOG=debug
```

### Custom Build Settings

Edit `rust_processor/Cargo.toml` for custom optimizations:

```toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization, slower compile
opt-level = 3           # Maximum optimization
strip = true            # Remove debug symbols
```

## Development Workflow

### Quick Edit-Test Cycle

```bash
# Edit Rust code
vim rust_processor/src/lib.rs

# Rebuild (fast debug build)
make build

# Test
python examples/basic_usage.py
```

### Production Build

```bash
# Full optimization build
make build-release

# Run benchmarks to verify performance
make benchmark-full
```

### Clean Build

```bash
# Remove all build artifacts
make clean

# Fresh build
make build
```

## Performance Tuning

### Optimization Checklist

1. **Use release builds** - `make build-release` (10-20x faster than debug)
2. **Batch operations** - Fewer Python↔Rust boundary crosses
3. **Use appropriate data structures** - Pre-allocate when possible
4. **Profile your code** - Find actual bottlenecks
5. **Increase dataset size** - Parallelization benefits increase

### Profiling

```bash
# Python profiling
python -m cProfile -s cumtime examples/benchmark.py

# Rust profiling (requires cargo-flamegraph)
cargo install flamegraph
cd rust_processor
cargo flamegraph --bin <your_binary>
```

## Getting Help

If you're still stuck:

1. **Check the error message carefully** - Often contains the solution
2. **Search GitHub issues** - Someone may have had the same problem
3. **Check PyO3 documentation** - https://pyo3.rs/
4. **Ask on forums** - Stack Overflow, Reddit r/rust
5. **Create an issue** - Provide full error message and environment details

### Providing Debug Information

When reporting issues, include:

```bash
# System information
uname -a                          # Linux/macOS
systeminfo                        # Windows

# Python information
python --version
pip list

# Rust information
rustc --version
cargo --version

# Build output
make clean
make build 2>&1 | tee build.log   # Save full build output
```

---

**Next Steps:**

- ✅ Setup complete? → [QUICKSTART.md](QUICKSTART.md)
- 🎯 Understand the code? → [NAVIGATION.md](NAVIGATION.md)
- 🚀 Learn Rust concepts? → [KEY_CONCEPTS.md](KEY_CONCEPTS.md)
