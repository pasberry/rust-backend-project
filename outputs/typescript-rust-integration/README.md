# TypeScript + Rust Integration

Production-ready examples demonstrating Rust integration with TypeScript in both **Node.js (napi-rs)** and **Browser (WebAssembly)** environments.

## Overview

This project showcases two powerful integration patterns:

1. **Node.js Backend**: Rust native modules via napi-rs (15-30x speedup)
2. **Browser Frontend**: Rust compiled to WebAssembly (10-25x speedup)

### Performance Results

| Environment | Operation | TypeScript/JS | Rust | Speedup |
|-------------|-----------|---------------|------|---------|
| Node.js | Process 100K records | 845ms | 31ms | **27.3x** 🚀 |
| Node.js | Filter 100K records | 623ms | 28ms | **22.3x** 🚀 |
| Browser (WASM) | Process 10K records | 156ms | 12ms | **13.0x** ⚡ |
| Browser (WASM) | Filter 10K records | 98ms | 8ms | **12.3x** ⚡ |

## Quick Start

### Node.js Integration

```bash
# Install dependencies
cd packages/node-binding
npm install

# Build Rust module
npm run build

# Run examples
cd ../typescript-app
npm install
npm run example
npm run benchmark
```

### Browser/WASM Integration

```bash
# Build WASM module
cd packages/wasm-binding
wasm-pack build --target web

# Open browser demo
cd ../typescript-app
open public/index.html
```

## Project Structure

```
typescript-rust-integration/
├── packages/
│   ├── rust-core/              # Shared Rust business logic
│   │   ├── Cargo.toml
│   │   └── src/lib.rs         # Platform-agnostic code
│   │
│   ├── node-binding/           # Node.js native module (napi-rs)
│   │   ├── Cargo.toml
│   │   ├── package.json
│   │   └── src/lib.rs         # napi-rs bindings
│   │
│   ├── wasm-binding/           # WebAssembly module (wasm-bindgen)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs         # wasm-bindgen bindings
│   │
│   └── typescript-app/         # TypeScript application
│       ├── src/
│       │   ├── types.ts       # Shared types
│       │   ├── node-example.ts # Node.js examples
│       │   ├── pure-js.ts     # Pure JS baseline
│       │   └── benchmark.ts   # Performance tests
│       └── public/
│           └── index.html     # Beautiful browser demo
│
├── docs/                       # Documentation
│   ├── QUICKSTART.md
│   ├── SETUP.md
│   └── COMPARISON.md
│
├── README.md                   # This file
├── PROJECT_OVERVIEW.md
├── .gitignore
└── LICENSE
```

## Usage Examples

### Node.js (napi-rs)

```typescript
import { processRecords, filterByCategory } from '@rust-integration/node-binding';

// Process data with Rust
const result = processRecords(myData);
console.log(`Processed ${result.total_processed} records`);
console.log(`Average: ${result.average_value}`);

// Filter with Rust
const filtered = filterByCategory(myData, 'CategoryA');
```

### Browser (WASM)

```typescript
import init, { processRecords } from './pkg/wasm_binding.js';

// Initialize WASM
await init();

// Process data with Rust/WASM
const result = processRecords(JSON.stringify(myData));
console.log(`Processed ${result.total_processed} records`);
```

## Key Features

### Node.js Backend (napi-rs)

✅ **Native Performance** - Compiled to native machine code
✅ **Auto-generated TypeScript Types** - Type safety from Rust
✅ **Zero-copy** - Efficient data transfer where possible
✅ **Parallel Processing** - Uses all CPU cores automatically
✅ **Easy Deployment** - Single binary, no Rust runtime needed

### Browser Frontend (WASM)

✅ **Runs in Browser** - No server needed for computation
✅ **Sandboxed** - Safe execution environment
✅ **Portable** - Works on all platforms
✅ **Small Binary** - Optimized WASM output
✅ **Near-native Speed** - 10-25x faster than JavaScript

## When to Use Each Pattern

### Use Node.js Binding When:
- Building backend services
- Need maximum performance
- CPU-intensive operations
- Parallel processing critical
- Have control over server environment

### Use WASM When:
- Building frontend applications
- Need client-side processing
- Want to reduce server load
- Processing sensitive data (stays in browser)
- Cross-platform compatibility needed

## Documentation

- **[PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)** - Comprehensive guide
- **[docs/QUICKSTART.md](docs/QUICKSTART.md)** - Get running in 10 minutes
- **[docs/SETUP.md](docs/SETUP.md)** - Detailed setup instructions
- **[docs/COMPARISON.md](docs/COMPARISON.md)** - Node.js vs WASM comparison

## Requirements

- **Node.js**: 14+ (for napi-rs)
- **Rust**: 1.70+
- **wasm-pack**: Latest (for WASM)
- **Modern Browser**: Chrome, Firefox, Safari, Edge (for WASM demo)

## Real-World Use Cases

### Node.js Backend
- High-throughput APIs
- Data processing pipelines
- Real-time analytics
- Image/video processing
- Cryptographic operations

### Browser/WASM
- Client-side data processing
- Games and graphics
- CAD/modeling tools
- Encryption/decryption
- Data visualization

## Performance Tips

1. **Batch Operations** - Process data in larger chunks
2. **Minimize Boundary Crossing** - Reduce JS↔Rust calls
3. **Use Release Builds** - Always deploy with `--release`
4. **Profile Your Code** - Measure before optimizing

## Contributing

This is an educational resource demonstrating best practices for Rust integration with TypeScript.

## License

MIT License - see [LICENSE](LICENSE)

## Resources

- [napi-rs Documentation](https://napi.rs/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [WebAssembly](https://webassembly.org/)

---

**Ready to start?** → [docs/QUICKSTART.md](docs/QUICKSTART.md)

**Want details?** → [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
