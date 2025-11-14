# Quick Start Guide

Get up and running with Rust + Python integration in **5 minutes**!

## Prerequisites

Make sure you have:
- Python 3.8+ installed (`python --version`)
- Rust 1.70+ installed (`rustc --version`)
  - If not: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## 1. Install Dependencies (30 seconds)

```bash
make install
```

This installs:
- `maturin` - Builds Rust extensions for Python
- `pytest` - Testing framework
- Development tools

**Alternative without Make:**
```bash
pip install -r requirements.txt
```

## 2. Build the Rust Module (1-2 minutes)

```bash
make build
```

This compiles the Rust code and makes it importable from Python.

**What's happening?**
- Rust code is compiled to a native shared library (.so on Linux, .dylib on macOS, .pyd on Windows)
- The library is placed where Python can import it
- You can now `import rust_processor` in Python!

**Alternative without Make:**
```bash
cd rust_processor
maturin develop
cd ..
```

**For production (optimized build):**
```bash
make build-release  # Takes longer, runs much faster
```

## 3. Run Examples (1 minute)

```bash
make example
```

This runs `examples/basic_usage.py` which demonstrates:
1. Parsing JSON logs
2. Validating log entries
3. Computing statistics
4. Filtering logs
5. Batch processing

**Expected output:**
```
======================================================================
  Rust + Python Integration: Basic Usage Examples
======================================================================

Example 1: Basic Log Parsing
======================================================================

✅ Successfully parsed 3 log entries
...
```

## 4. See the Performance Difference (1 minute)

```bash
make benchmark
```

This runs a quick benchmark comparing Python vs Rust performance.

**Expected output:**
```
⚡ Quick Performance Benchmark
================================================================================

Benchmarking: Compute Statistics (10,000 records)

Operation: Compute Stats
  Dataset: 10,000 records
  Python:   850.23ms (11,762 records/sec)
  Rust:     28.45ms (351,493 records/sec)
  Speedup:  29.9x 🚀

================================================================================
  Result: Rust is 29.9x faster! 🚀
================================================================================
```

## 5. Try Your Own Code

Create a file `test_integration.py`:

```python
import json
import rust_processor

# Create some sample logs
logs = [
    json.dumps({
        "timestamp": "2024-01-15T10:30:00Z",
        "level": "ERROR",
        "message": "Database connection failed",
        "duration_ms": 1250.5,
        "status_code": 500
    }),
    json.dumps({
        "timestamp": "2024-01-15T10:30:01Z",
        "level": "INFO",
        "message": "Request processed",
        "duration_ms": 45.2,
        "status_code": 200
    })
]

# Use Rust to compute statistics
stats = rust_processor.compute_stats(logs)

print(f"Total logs: {stats.total_count}")
print(f"Errors: {stats.error_count}")
print(f"Average duration: {stats.avg_duration_ms:.2f}ms")
print(f"P95 duration: {stats.p95_duration_ms:.2f}ms")
```

Run it:
```bash
python test_integration.py
```

## Next Steps

### Learn More

- **See more examples**: `examples/basic_usage.py` has 5 detailed examples
- **Understand the code**: Read `rust_processor/src/lib.rs` (well-commented)
- **Production patterns**: Check `python_orchestrator/log_processor/pipeline.py`

### Run Full Benchmarks

```bash
make benchmark-full
```

Tests with 1K, 10K, and 100K record datasets. Shows comprehensive performance comparison.

### Generate Test Data

```bash
make data
```

Creates sample data files:
- `data/sample_1k.jsonl` - 1,000 records
- `data/sample_10k.jsonl` - 10,000 records
- `data/sample_100k.jsonl` - 100,000 records
- `data/sample_1m.jsonl` - 1,000,000 records

Use these for testing and benchmarking.

### Explore Documentation

- **[SETUP.md](SETUP.md)** - Detailed setup, troubleshooting
- **[PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)** - Comprehensive overview
- **[KEY_CONCEPTS.md](KEY_CONCEPTS.md)** - Rust concepts for Python developers
- **[NAVIGATION.md](NAVIGATION.md)** - Navigate the codebase
- **[LANGUAGE_COMPARISON.md](LANGUAGE_COMPARISON.md)** - JS vs Python vs Rust

## Common Issues

### ImportError: No module named 'rust_processor'

**Solution:** You need to build the Rust module first.
```bash
make build
```

### maturin: command not found

**Solution:** Install dependencies.
```bash
make install
# or
pip install maturin
```

### Build fails with "cargo not found"

**Solution:** Install Rust.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Performance not as expected

**Solution:** Build in release mode for full optimization.
```bash
make build-release
```

Debug builds are 10-20x slower than release builds!

## All Available Commands

```bash
make install        # Install dependencies
make build          # Build in debug mode (fast compile)
make build-release  # Build in release mode (optimized)
make example        # Run basic examples
make benchmark      # Quick benchmark
make benchmark-full # Comprehensive benchmark
make data           # Generate sample data
make test           # Run tests
make format         # Format Python code
make clean          # Remove build artifacts
make help           # Show all commands
```

## Quick Reference Card

| Task | Command |
|------|---------|
| First time setup | `make install && make build` |
| Rebuild after Rust changes | `make build` |
| Run examples | `make example` |
| See performance | `make benchmark` |
| Production build | `make build-release` |
| Clean everything | `make clean` |

## You're Ready!

You now have a working Rust + Python integration!

**What you can do:**
- ✅ Import and use Rust functions from Python
- ✅ Process data 20-50x faster than pure Python
- ✅ Understand the basic integration pattern
- ✅ Build and deploy your own Rust extensions

**Where to go from here:**
1. Read through `examples/basic_usage.py` to see more patterns
2. Study `rust_processor/src/lib.rs` to understand the Rust side
3. Check `python_orchestrator/log_processor/pipeline.py` for production patterns
4. Adapt this template to your own use case!

---

**Having issues?** → See [SETUP.md](SETUP.md) for detailed troubleshooting

**Want to understand Rust?** → See [KEY_CONCEPTS.md](KEY_CONCEPTS.md)

**Ready for production?** → See [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
