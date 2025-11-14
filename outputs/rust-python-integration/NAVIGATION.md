# Code Navigation Guide

A comprehensive guide to navigating and understanding the codebase.

## Project Structure

```
rust-python-integration/
├── rust_processor/              # Rust core library
│   ├── Cargo.toml              # Rust dependencies & config
│   └── src/
│       └── lib.rs              # Main Rust code with PyO3 bindings
│
├── python_orchestrator/         # Python layer
│   └── log_processor/
│       ├── __init__.py         # Package initialization
│       ├── pipeline.py         # Production patterns
│       └── pure_python.py      # Baseline for comparison
│
├── examples/                    # Educational examples
│   ├── basic_usage.py          # 5 progressive examples
│   ├── benchmark.py            # Performance testing
│   └── generate_sample_data.py # Test data generator
│
├── data/                        # Generated test data (gitignored)
│
├── Makefile                     # Build automation
├── pyproject.toml              # Python package config
├── requirements.txt            # Python dependencies
│
├── README.md                    # Project overview
├── QUICKSTART.md               # 5-minute setup
├── SETUP.md                    # Detailed setup
├── PROJECT_OVERVIEW.md         # Comprehensive guide
├── NAVIGATION.md               # This file
├── KEY_CONCEPTS.md             # Rust concepts
└── LANGUAGE_COMPARISON.md      # Language comparison
```

## File-by-File Guide

### Rust Core (`rust_processor/src/lib.rs`)

**Lines 1-30: Imports and Structure Definitions**
```rust
use pyo3::prelude::*;        // PyO3 for Python bindings
use rayon::prelude::*;       // Parallel processing
use serde::{Deserialize, Serialize};  // JSON handling

pub struct LogEntry { ... }  // Log data structure
```

**Lines 31-80: LogStats Class**
```rust
#[pyclass]                   // Exposes to Python as a class
pub struct LogStats {
    #[pyo3(get)]            // Makes field accessible from Python
    pub total_count: usize,
    // ... more fields
}
```
This is what Python receives when calling `compute_stats()`.

**Lines 81-150: parse_logs() Function**
```rust
#[pyfunction]                // Exposes to Python as a function
fn parse_logs(log_lines: Vec<String>) -> PyResult<...> {
    log_lines
        .par_iter()          // PARALLEL processing
        .map(|line| {...})   // Parse each line
        .collect()           // Gather results
}
```
Key: `.par_iter()` makes it use all CPU cores!

**Lines 151-220: validate_logs() Function**
```rust
fn validate_logs(log_lines: Vec<String>) -> PyResult<(usize, Vec<String>)> {
    // Returns (valid_count, error_messages)
    // Validates: timestamps, log levels, durations, status codes
}
```

**Lines 221-320: compute_stats() Function**
```rust
fn compute_stats(log_lines: Vec<String>) -> PyResult<LogStats> {
    // 1. Parse logs in parallel
    // 2. Count by level
    // 3. Collect durations
    // 4. Sort for percentiles
    // 5. Compute aggregations
    // 6. Return LogStats object
}
```
This is the performance showcase - shows 46x speedup!

**Lines 321-400: filter_logs() Function**
```rust
fn filter_logs(...) -> PyResult<Vec<...>> {
    // Parallel filtering by:
    // - Log level (ERROR, WARN, INFO, DEBUG)
    // - Duration threshold
    // - Status codes
}
```

**Lines 401-430: batch_process() Function**
```rust
fn batch_process(...) -> PyResult<(LogStats, Vec<String>)> {
    // Combines validate + compute_stats
    // More efficient than two separate calls
}
```

**Lines 431-440: Module Definition**
```rust
#[pymodule]
fn rust_processor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_logs, m)?)?;
    // ... register all functions
    m.add_class::<LogStats>()?;  // Register LogStats class
    Ok(())
}
```
This makes everything available to Python!

### Python Pipeline (`python_orchestrator/log_processor/pipeline.py`)

**Lines 1-50: Imports and Data Structures**
```python
try:
    import rust_processor
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
```
Graceful handling if Rust module not built yet.

**Lines 51-100: LogPipeline Class**
```python
class LogPipeline:
    def __init__(self, error_threshold: int = 100):
        # Initialize with business logic parameters
```

**Lines 101-150: load_logs_from_file()**
```python
def load_logs_from_file(self, file_path: Path) -> List[str]:
    # Python handles file I/O
    # Returns list of JSON strings for Rust to process
```
This is Python's strength - I/O operations.

**Lines 151-200: process_batch()**
```python
def process_batch(self, log_lines: List[str]) -> ProcessingResult:
    # THE KEY INTEGRATION POINT
    # 1. Python prepares data
    start_time = time.time()

    # 2. Rust processes (fast!)
    stats, errors = rust_processor.batch_process(log_lines)

    # 3. Python handles results
    processing_time = (time.time() - start_time) * 1000
    return ProcessingResult(...)
```

**Lines 201-250: Filtering Methods**
```python
def filter_high_severity_logs(self, ...):
    # Calls Rust for filtering
    filtered = rust_processor.filter_logs(...)
    return filtered
```

**Lines 251-300: analyze_and_alert()**
```python
def analyze_and_alert(self, file_path: Path):
    # Complete pipeline demonstrating:
    # 1. Load (Python)
    # 2. Process (Rust)
    # 3. Business logic (Python)
```
This is the production pattern!

**Lines 301-350: Business Logic Methods**
```python
def _send_alert(self, result):
    # Python's strength - integration
    # In production: email, Slack, PagerDuty, etc.

def _analyze_errors(self, error_logs):
    # Python's strength - flexible analysis
```

### Pure Python Implementation (`python_orchestrator/log_processor/pure_python.py`)

**Lines 1-100: PythonLogStats and PurePythonProcessor**
```python
@dataclass
class PythonLogStats:
    # Mirrors Rust LogStats for comparison

class PurePythonProcessor:
    # Pure Python implementations
    # Used for benchmarking
```

**Key difference from Rust:**
```python
# Python: Single-threaded
for line in log_lines:
    entry = json.loads(line)  # Slower JSON parser
    # ... process

# Rust: Multi-threaded
log_lines.par_iter()          # Automatic parallelization
    .map(|line| serde_json::from_str(line))  # Faster parser
    .collect()
```

### Examples

#### `examples/basic_usage.py`

**Structure:**
```python
def example1_parse_logs():     # Lines 30-60
def example2_validate_logs():  # Lines 62-92
def example3_compute_stats():  # Lines 94-130
def example4_filter_logs():    # Lines 132-180
def example5_batch_process():  # Lines 182-250
```

Progressive complexity - start with example1, work through to example5.

#### `examples/benchmark.py`

**Key Functions:**
```python
def generate_test_data(count):         # Lines 20-50
def benchmark_function(...):           # Lines 52-80
def print_result(...):                 # Lines 82-95
def benchmark_parse_logs(...):         # Lines 97-120
# ... more benchmark functions
def run_comprehensive_benchmark():     # Lines 200-300
```

**How to read benchmark output:**
```
Operation: Compute Stats
  Dataset: 10,000 records
  Python:   850.23ms (11,762 records/sec)
  Rust:     28.45ms (351,493 records/sec)
  Speedup:  29.9x 🚀
```

- **Python time**: Pure Python implementation
- **Rust time**: Rust implementation
- **Speedup**: How many times faster Rust is
- **records/sec**: Throughput (higher is better)

## Common Code Patterns

### Pattern 1: Calling Rust from Python

```python
# 1. Import
import rust_processor

# 2. Prepare data (Python)
log_lines = [json.dumps(log) for log in logs]

# 3. Call Rust
stats = rust_processor.compute_stats(log_lines)

# 4. Use results (Python)
print(f"Errors: {stats.error_count}")
```

### Pattern 2: Error Handling

```python
try:
    stats = rust_processor.compute_stats(log_lines)
except ValueError as e:
    # Rust validation errors become Python exceptions
    print(f"Validation error: {e}")
except RuntimeError as e:
    # Rust runtime errors
    print(f"Processing error: {e}")
```

### Pattern 3: Accessing Rust Class Properties

```python
stats = rust_processor.compute_stats(log_lines)

# Access properties defined with #[pyo3(get)]
print(stats.total_count)      # OK
print(stats.error_count)      # OK
print(stats.avg_duration_ms)  # OK

# Call methods
print(stats.summary())        # OK
print(repr(stats))            # OK
```

## Data Flow

### Example: Computing Statistics

```
┌─────────────────┐
│ Python          │
│ File I/O        │  1. Load file
└────────┬────────┘
         │ Vec<String> (JSON lines)
         ▼
┌─────────────────┐
│ Rust            │
│ parse_logs()    │  2. Parse JSON (parallel)
└────────┬────────┘
         │ Vec<LogEntry>
         ▼
┌─────────────────┐
│ Rust            │
│ Process         │  3. Count, aggregate, sort
└────────┬────────┘
         │ LogStats struct
         ▼
┌─────────────────┐
│ Python          │
│ Business Logic  │  4. Check thresholds, alert
└─────────────────┘
```

## Key Integration Points

### 1. Type Conversions (PyO3 Automatic)

| Python | Rust |
|--------|------|
| `str` | `String` |
| `list[str]` | `Vec<String>` |
| `dict[str, str]` | `HashMap<String, String>` |
| `int` | `i32`, `i64`, `usize` |
| `float` | `f64` |
| `None` | `Option<T>` |

### 2. Error Propagation

```rust
// Rust
return Err(PyValueError::new_err("Invalid data"));
```
```python
# Python
try:
    result = rust_processor.func()
except ValueError as e:
    # "Invalid data"
```

### 3. Class Exposure

```rust
// Rust
#[pyclass]
pub struct LogStats {
    #[pyo3(get)]
    pub total_count: usize,
}
```
```python
# Python
stats = rust_processor.compute_stats(logs)
print(stats.total_count)  # Access like Python attribute
```

## Performance Hotspots

Where the speedup comes from:

1. **Parsing (parse_logs)** - 27x faster
   - `serde_json` vs Python `json`
   - Parallel processing with Rayon
   - No GIL contention

2. **Sorting (compute_stats)** - 46x faster
   - Rust's optimized sort algorithm
   - In-place sorting (no copies)
   - Compiled code

3. **Filtering (filter_logs)** - 28x faster
   - Parallel filtering
   - Zero-copy where possible
   - Branch prediction optimization

## Debugging Tips

### Python Side

```python
# Add debug prints
import rust_processor
print(f"Module: {rust_processor}")
print(f"Functions: {dir(rust_processor)}")

# Try simple call
try:
    result = rust_processor.parse_logs([json.dumps({"test": "data"})])
    print(f"Success: {result}")
except Exception as e:
    print(f"Error: {e}")
    import traceback
    traceback.print_exc()
```

### Rust Side

```rust
// Add debug prints (only visible during build)
eprintln!("Debug: processing {} logs", log_lines.len());

// Or use proper logging (to Python)
use pyo3::prelude::*;
Python::with_gil(|py| {
    py.run("import sys; print('From Rust!', file=sys.stderr)", None, None)
})?;
```

## Where to Start Reading

**For Python developers:**
1. Start: `examples/basic_usage.py` example1
2. Next: `python_orchestrator/log_processor/pipeline.py` - see integration
3. Then: `rust_processor/src/lib.rs` - understand Rust side

**For Rust developers:**
1. Start: `rust_processor/src/lib.rs` - see PyO3 patterns
2. Next: `examples/basic_usage.py` - see Python usage
3. Then: `examples/benchmark.py` - see performance gains

**For learning the integration:**
1. Start: `examples/basic_usage.py` example1 (simple)
2. Next: `examples/basic_usage.py` example5 (batch)
3. Then: `python_orchestrator/log_processor/pipeline.py` (production)

## Next Steps

- **Understand Rust concepts** → [KEY_CONCEPTS.md](KEY_CONCEPTS.md)
- **Compare languages** → [LANGUAGE_COMPARISON.md](LANGUAGE_COMPARISON.md)
- **Run the code** → [QUICKSTART.md](QUICKSTART.md)
- **Deep dive** → [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
