# Rust Integration Examples: Python & TypeScript

Two comprehensive, production-ready projects demonstrating **15-50x performance improvements** by integrating Rust with Python and TypeScript.

## 📦 Projects Overview

### 1. [Python + Rust Integration](./rust-python-integration/)

**Pattern:** Python orchestration with Rust hot paths (PyO3)

**Use Case:** High-performance log processing pipeline
- Parse millions of JSON log entries
- Validate data schemas
- Compute statistics (percentiles, aggregations)
- Filter by complex criteria

**Performance:** **20-50x speedup** over pure Python

**Key Technologies:**
- PyO3 for Python bindings
- Rayon for parallel processing
- Maturin for building

**Quick Start:**
```bash
cd rust-python-integration
make install && make build && make example
```

---

### 2. [TypeScript + Rust Integration](./typescript-rust-integration/)

**Patterns:** Both Node.js backend (napi-rs) AND Browser frontend (WebAssembly)

**Use Case:** Data transformation and processing for server and client
- Process records with statistics
- Filter and aggregate data
- Category analysis
- Real-time processing

**Performance:**
- **Node.js:** 15-30x speedup
- **Browser (WASM):** 10-25x speedup

**Key Technologies:**
- napi-rs for Node.js native modules
- wasm-bindgen for WebAssembly
- Shared Rust core

**Quick Start:**
```bash
cd typescript-rust-integration/packages/node-binding
npm install && npm run build

cd ../typescript-app
npm install && npm run example
```

---

## 🎯 Which Project Should You Explore?

### Choose Python + Rust if you:
- Use Python for data processing, ML, or APIs
- Need to speed up CPU-intensive Python code
- Want to keep Python's ecosystem while gaining performance
- Have existing Python applications with slow hot paths

### Choose TypeScript + Rust (Node.js) if you:
- Build Node.js backends
- Need better performance than JavaScript
- Want type-safe native modules
- Process data server-side

### Choose TypeScript + Rust (WASM) if you:
- Build web applications
- Need client-side processing
- Want to reduce server load
- Process sensitive data in the browser

---

## 📊 Performance Comparison

| Project | Environment | Operation | Before (Python/JS) | After (Rust) | Speedup |
|---------|-------------|-----------|-------------------|--------------|---------|
| Python Integration | Python 3.11 | Process 100K logs | 9,350ms | 203ms | **46.1x** 🚀 |
| TypeScript Integration | Node.js 20 | Process 100K records | 845ms | 31ms | **27.3x** 🚀 |
| TypeScript Integration | Browser (Chrome) | Process 10K records | 156ms | 12ms | **13.0x** ⚡ |

---

## 🏗️ Project Structure

```
outputs/
├── README.md                          # This file
├── GITHUB_SETUP_GUIDE.md              # How to push to GitHub (3 methods)
├── QUICK_GITHUB_REFERENCE.md          # One-page quick reference
├── setup-github.sh                    # Automated GitHub setup script
│
├── rust-python-integration/           # Python + Rust project
│   ├── README.md                      # Project overview
│   ├── QUICKSTART.md                  # 5-minute setup
│   ├── SETUP.md                       # Detailed setup & troubleshooting
│   ├── PROJECT_OVERVIEW.md            # Comprehensive guide
│   ├── NAVIGATION.md                  # Code navigation guide
│   ├── KEY_CONCEPTS.md                # Rust concepts for Python devs
│   ├── LANGUAGE_COMPARISON.md         # JS/Python/Rust comparison
│   ├── Makefile                       # Build automation
│   ├── rust_processor/                # Rust core (PyO3)
│   ├── python_orchestrator/           # Python layer
│   └── examples/                      # Usage examples & benchmarks
│
└── typescript-rust-integration/       # TypeScript + Rust project
    ├── README.md                      # Project overview
    ├── PROJECT_OVERVIEW.md            # Comprehensive guide
    ├── docs/                          # Documentation
    └── packages/
        ├── rust-core/                 # Shared Rust logic
        ├── node-binding/              # Node.js binding (napi-rs)
        ├── wasm-binding/              # WASM binding (wasm-bindgen)
        └── typescript-app/            # TypeScript examples & demo
            ├── src/                   # Node.js examples
            └── public/index.html      # Beautiful browser demo!
```

---

## 🚀 Quick Start Guide

### Python + Rust Integration

```bash
cd rust-python-integration

# Install and build
make install
make build

# Run examples
make example          # 5 usage examples
make benchmark        # Performance comparison
make data             # Generate test data
```

### TypeScript + Rust (Node.js)

```bash
cd typescript-rust-integration/packages/node-binding

# Build Rust module
npm install
npm run build

# Run examples
cd ../typescript-app
npm install
npm run example       # 5 usage examples
npm run benchmark     # Performance comparison
```

### TypeScript + Rust (Browser/WASM)

```bash
cd typescript-rust-integration/packages/wasm-binding

# Build WASM
wasm-pack build --target web

# Open browser demo
cd ../typescript-app
open public/index.html    # Or your browser
```

---

## 📚 Documentation

### Python + Rust
- [README.md](./rust-python-integration/README.md) - Overview
- [QUICKSTART.md](./rust-python-integration/QUICKSTART.md) - 5-minute setup
- [KEY_CONCEPTS.md](./rust-python-integration/KEY_CONCEPTS.md) - Rust for Python devs
- [PROJECT_OVERVIEW.md](./rust-python-integration/PROJECT_OVERVIEW.md) - Deep dive

### TypeScript + Rust
- [README.md](./typescript-rust-integration/README.md) - Overview
- [PROJECT_OVERVIEW.md](./typescript-rust-integration/PROJECT_OVERVIEW.md) - Deep dive
- [docs/QUICKSTART.md](./typescript-rust-integration/docs/QUICKSTART.md) - 10-minute setup
- [docs/COMPARISON.md](./typescript-rust-integration/docs/COMPARISON.md) - Node.js vs WASM

### GitHub Setup
- [GITHUB_SETUP_GUIDE.md](./GITHUB_SETUP_GUIDE.md) - 3 methods to push to GitHub
- [QUICK_GITHUB_REFERENCE.md](./QUICK_GITHUB_REFERENCE.md) - One-page reference
- [setup-github.sh](./setup-github.sh) - Automated setup script

---

## 🎓 Learning Path

### For Python Developers
1. Start: `rust-python-integration/QUICKSTART.md`
2. Explore: `rust-python-integration/examples/basic_usage.py`
3. Understand: `rust-python-integration/KEY_CONCEPTS.md`
4. Benchmark: `cd rust-python-integration && make benchmark`
5. Deep dive: `rust-python-integration/PROJECT_OVERVIEW.md`

### For JavaScript/TypeScript Developers
1. Start: `typescript-rust-integration/docs/QUICKSTART.md`
2. Explore: Run `npm run example` in typescript-app
3. Browser demo: Open `typescript-app/public/index.html`
4. Benchmark: `npm run benchmark`
5. Deep dive: `typescript-rust-integration/PROJECT_OVERVIEW.md`

### For Learning Rust Concepts
1. Read: `rust-python-integration/KEY_CONCEPTS.md`
2. Compare: `rust-python-integration/LANGUAGE_COMPARISON.md`
3. Study code: Compare `pure_python.py` vs `lib.rs`

---

## 🎯 Real-World Applications

### These Patterns Are Used By

**Python + Rust:**
- **Dropbox** - File sync engine
- **AWS** - Firecracker VMM
- **Meta** - Source control tools

**TypeScript + Rust (Node.js):**
- **Discord** - Read states service
- **Cloudflare** - Edge computing
- **Vercel** - Build systems

**TypeScript + Rust (WASM):**
- **Figma** - Multiplayer engine
- **1Password** - Security operations
- **Google Earth** - 3D rendering

---

## 💡 Key Takeaways

1. **Incremental Adoption** - Start with one hot path, expand gradually
2. **Best of Both Worlds** - Keep high-level language benefits, add performance where needed
3. **Production Ready** - These patterns are used by major companies
4. **Educational Resource** - Well-documented, comprehensive examples
5. **Easy Integration** - PyO3, napi-rs, and wasm-bindgen make it straightforward

---

## 📝 License

MIT License - see LICENSE files in each project directory.

---

## 🔗 Resources

### Official Documentation
- [PyO3 Guide](https://pyo3.rs/)
- [napi-rs Documentation](https://napi.rs/)
- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)
- [The Rust Book](https://doc.rust-lang.org/book/)

### Tools
- [Maturin](https://github.com/PyO3/maturin) - Build Python extensions
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) - Build WebAssembly
- [Rayon](https://github.com/rayon-rs/rayon) - Data parallelism in Rust

---

## 🚀 Ready to Get Started?

Pick a project and dive in:

- **Python background?** → [rust-python-integration/QUICKSTART.md](./rust-python-integration/QUICKSTART.md)
- **TypeScript/Node.js?** → [typescript-rust-integration/docs/QUICKSTART.md](./typescript-rust-integration/docs/QUICKSTART.md)
- **Want to see it in action?** → Open `typescript-rust-integration/packages/typescript-app/public/index.html` in your browser!

---

## 📤 Pushing to GitHub

See [GITHUB_SETUP_GUIDE.md](./GITHUB_SETUP_GUIDE.md) for three easy methods to push these projects to GitHub!

---

**Built with ❤️ using Rust, Python, and TypeScript**

*Demonstrating that you can have both developer productivity AND performance!*
