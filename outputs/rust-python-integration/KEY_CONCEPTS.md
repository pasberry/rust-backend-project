# Key Rust Concepts for Python/JavaScript Developers

A gentle introduction to Rust's unique features, explained for developers coming from Python or JavaScript.

## Table of Contents

- [The Big Picture](#the-big-picture)
- [Ownership](#ownership)
- [Borrowing](#borrowing)
- [Lifetimes](#lifetimes)
- [Type System](#type-system)
- [Error Handling](#error-handling)
- [Pattern Matching](#pattern-matching)
- [Traits](#traits)
- [Common Gotchas](#common-gotchas)

## The Big Picture

### Why Rust is Different

| Aspect | Python/JavaScript | Rust |
|--------|------------------|------|
| **Memory** | Garbage collected | Ownership system |
| **When errors caught** | Runtime | Compile time |
| **Typing** | Dynamic | Static |
| **Safety** | Runtime checks | Compile time guarantees |
| **Performance** | Interpreted/JIT | Compiled to native code |
| **Concurrency** | GIL/Single-threaded (mostly) | Fearless concurrency |

### Rust's Safety Promise

**Three guarantees:**
1. **No null pointer errors** - `Option<T>` instead of null
2. **No data races** - Ownership system prevents concurrent access
3. **No use-after-free** - Compiler tracks memory lifetime

All checked **at compile time** with **zero runtime cost**!

## Ownership

### The Core Concept

**Rule:** Every value has exactly one owner. When the owner goes out of scope, the value is dropped (freed).

### Python/JavaScript Way

```python
# Python: References everywhere, GC cleans up
def process():
    data = [1, 2, 3]       # Created
    copy = data            # Another reference
    process_data(data)     # Another reference
    # GC figures out when to free memory
```

```javascript
// JavaScript: Similar to Python
let data = [1, 2, 3];
let copy = data;           // Another reference
processData(data);         // Another reference
// GC handles cleanup
```

### Rust Way

```rust
fn process() {
    let data = vec![1, 2, 3];   // data OWNS the vector

    let moved = data;            // Ownership MOVED to 'moved'
    // println!("{:?}", data);   // ERROR! data no longer valid

    process_data(moved);         // Ownership MOVED to function
    // println!("{:?}", moved);  // ERROR! moved no longer valid

}  // 'moved' was moved, so nothing to drop here
```

**Key difference:** Move, not copy!

### Why This Matters

```python
# Python: Easy to create bugs
data = {"key": "value"}
process(data)
data["key"] = "new"  # Did process modify it? Hard to know!
```

```rust
// Rust: Clear ownership
let data = HashMap::new();
process(data);       // 'data' is moved
// data.insert(...);  // ERROR! Compiler won't let you!
```

**Result:** No confusion about who owns what. No hidden mutations.

### Solutions in Rust

#### 1. Clone (if you need a copy)

```rust
let data = vec![1, 2, 3];
let copy = data.clone();     // Explicit copy
process_data(data);          // Original moved
println!("{:?}", copy);      // Copy still valid
```

#### 2. Borrow (if you just need to read)

```rust
let data = vec![1, 2, 3];
process_data(&data);         // Borrow (read-only)
println!("{:?}", data);      // Still valid!
```

## Borrowing

### Immutable Borrows (References)

**Python equivalent:** Passing by reference, but read-only

```rust
fn print_length(s: &String) {  // &String = borrow
    println!("Length: {}", s.len());
    // s.push_str("x");  // ERROR! Can't modify through & borrow
}

fn main() {
    let text = String::from("hello");
    print_length(&text);  // Pass a borrow
    println!("{}", text); // Still valid!
}
```

### Mutable Borrows

**Python equivalent:** Passing by reference, can modify

```rust
fn add_exclamation(s: &mut String) {  // &mut String = mutable borrow
    s.push_str("!");  // Can modify
}

fn main() {
    let mut text = String::from("hello");
    add_exclamation(&mut text);
    println!("{}", text);  // "hello!"
}
```

### Borrowing Rules

**The key rules:**
1. You can have **unlimited immutable borrows** OR **one mutable borrow**
2. Never both at the same time
3. Borrows must not outlive the owner

```rust
let mut data = vec![1, 2, 3];

let r1 = &data;     // OK: immutable borrow
let r2 = &data;     // OK: multiple immutable borrows
let r3 = &mut data; // ERROR! Can't have &mut while & exists

println!("{}, {}", r1, r2);  // Using immutable borrows

// r1 and r2 no longer used, so:
let r3 = &mut data; // OK now!
r3.push(4);
```

### Why This Prevents Bugs

```python
# Python: Iterator invalidation bug
data = [1, 2, 3]
for item in data:
    if item == 2:
        data.append(4)  # BUG! Modifying while iterating
```

```rust
// Rust: Compiler prevents this
let mut data = vec![1, 2, 3];
for item in &data {              // Immutable borrow
    if *item == 2 {
        data.push(4);            // ERROR! Can't mutate while borrowed
    }
}
```

**The compiler forces you to write correct code!**

### PyO3 and Borrowing

In our project:

```rust
#[pyfunction]
fn parse_logs(log_lines: Vec<String>) -> PyResult<Vec<...>> {
    log_lines.par_iter()  // Borrow each element
             .map(|line| {  // |line| is &String (borrowed)
                 // Process line
             })
             .collect()
}
```

The `par_iter()` creates immutable borrows, allowing parallel safe access!

## Lifetimes

### What Are Lifetimes?

**Lifetimes** ensure references don't outlive the data they point to.

### Python/JavaScript: Runtime Error

```python
def get_first(data):
    return data[0]  # Returns reference

result = get_first([1, 2, 3])  # [1, 2, 3] is temporary
# result still points to freed memory (if Python didn't use GC)
```

Python's GC saves you, but has performance cost.

### Rust: Compile Time Guarantee

```rust
fn get_first<'a>(data: &'a Vec<i32>) -> &'a i32 {
    &data[0]
}

fn main() {
    let nums = vec![1, 2, 3];
    let first = get_first(&nums);  // Borrow nums
    println!("{}", first);         // Use first
}  // nums and first dropped together - safe!
```

The `'a` lifetime annotation says: "The returned reference lives as long as the input reference."

### Common Lifetime Pattern

```rust
// Explicit lifetime
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Usage
let string1 = String::from("long");
let result;
{
    let string2 = String::from("short");
    result = longest(&string1, &string2);
    // result valid here
}
// result NOT valid here - string2 was dropped!
```

### Good News for PyO3

**You usually don't need to worry about lifetimes** when writing PyO3 code! PyO3 handles it for you:

```rust
#[pyfunction]
fn process(data: Vec<String>) -> PyResult<String> {
    // No lifetime annotations needed!
    Ok(data[0].clone())
}
```

## Type System

### Static Types

**Python/JavaScript:**
```python
def process(data):  # What type is data?
    return data + 1  # What if data is a string?
```

**Rust:**
```rust
fn process(data: i32) -> i32 {  // Types are explicit
    data + 1  // Compiler knows this is safe
}
```

### Type Inference

Rust infers types when it can:

```rust
let x = 5;              // Inferred as i32
let y = 5.0;            // Inferred as f64
let text = "hello";     // Inferred as &str
let vec = vec![1,2,3];  // Inferred as Vec<i32>
```

### Common Types

| Rust Type | Python Equivalent | Description |
|-----------|------------------|-------------|
| `i32`, `i64` | `int` | Integers |
| `f64` | `float` | Floating point |
| `bool` | `bool` | Boolean |
| `String` | `str` | Owned string |
| `&str` | `str` | String slice (borrowed) |
| `Vec<T>` | `list` | Growable array |
| `HashMap<K,V>` | `dict` | Hash map |
| `Option<T>` | `T \| None` | Maybe value |
| `Result<T, E>` | (no equivalent) | Success or error |

### Option: No More Null!

**Python:**
```python
def find_user(id):
    user = db.get(id)
    if user is None:  # Runtime check
        return None
    return user.name
```

**Rust:**
```rust
fn find_user(id: i32) -> Option<String> {
    db.get(id).map(|user| user.name)
}

// Usage
match find_user(123) {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}
```

**Compiler forces you to handle None!** No null pointer exceptions.

## Error Handling

### Python: Exceptions

```python
def divide(a, b):
    if b == 0:
        raise ValueError("Division by zero")
    return a / b

try:
    result = divide(10, 0)
except ValueError as e:
    print(f"Error: {e}")
```

### Rust: Result Type

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Usage
match divide(10, 0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

### The `?` Operator

Rust's "try" operator:

```rust
fn process() -> Result<i32, String> {
    let a = might_fail()?;  // Returns Err early if it fails
    let b = also_might_fail()?;
    Ok(a + b)
}
```

**Equivalent Python:**
```python
def process():
    try:
        a = might_fail()
        b = also_might_fail()
        return a + b
    except Exception as e:
        raise e
```

### PyO3 Error Handling

In our project:

```rust
#[pyfunction]
fn compute_stats(logs: Vec<String>) -> PyResult<LogStats> {
    if logs.is_empty() {
        return Err(PyValueError::new_err("No logs"));
    }
    // Process...
    Ok(stats)
}
```

Python receives:
```python
try:
    stats = rust_processor.compute_stats([])
except ValueError as e:
    print(e)  # "No logs"
```

## Pattern Matching

### JavaScript/Python

```python
# Python
if level == "ERROR":
    handle_error()
elif level == "WARN":
    handle_warn()
elif level == "INFO":
    handle_info()
else:
    handle_debug()
```

### Rust

```rust
match level {
    "ERROR" => handle_error(),
    "WARN" => handle_warn(),
    "INFO" => handle_info(),
    _ => handle_debug(),  // _ = default
}
```

### Powerful Pattern Matching

```rust
// Match on Option
match user {
    Some(u) => println!("User: {}", u.name),
    None => println!("No user"),
}

// Match on Result
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}

// Match with conditions
match age {
    0..=12 => "child",
    13..=19 => "teen",
    20..=65 => "adult",
    _ => "senior",
}
```

**The compiler ensures you handle all cases!**

## Traits

### Like Interfaces

**TypeScript:**
```typescript
interface Drawable {
    draw(): void;
}

class Circle implements Drawable {
    draw() { /* ... */ }
}
```

**Rust:**
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { }

impl Drawable for Circle {
    fn draw(&self) { /* ... */ }
}
```

### Common Traits in Our Project

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry { ... }
```

- `Debug` - Can print with `{:?}`
- `Clone` - Can call `.clone()`
- `Serialize` - Can convert to JSON (serde)
- `Deserialize` - Can parse from JSON (serde)

## Common Gotchas

### 1. String vs &str

```rust
let s1: String = String::from("hello");  // Owned
let s2: &str = "hello";                  // Borrowed

// Converting
let owned: String = s2.to_string();
let borrowed: &str = &s1;
```

**Rule of thumb:** Use `&str` for parameters, `String` for owned data.

### 2. Clone vs Copy

```rust
// Copy (cheap, automatic)
let x = 5;
let y = x;  // x is still valid

// Clone (explicit, potentially expensive)
let s1 = String::from("hello");
let s2 = s1.clone();  // Both valid
```

### 3. Iterator Consumption

```rust
let nums = vec![1, 2, 3];

// This consumes the vector
for num in nums {
    println!("{}", num);
}
// println!("{:?}", nums);  // ERROR! nums was consumed

// This borrows
for num in &nums {
    println!("{}", num);
}
println!("{:?}", nums);  // OK! nums still valid
```

### 4. Mutable State

```rust
let x = 5;
x = 6;  // ERROR! x is immutable

let mut y = 5;
y = 6;  // OK! y is mutable
```

**Everything is immutable by default!**

## Mental Model Shift

### Python/JavaScript Mindset → Rust Mindset

| Python/JS | Rust |
|-----------|------|
| "Pass by reference" | "Move or borrow?" |
| "Hope no null errors" | "Compiler guarantees no null" |
| "Try and catch errors" | "Handle all cases at compile time" |
| "GC will clean up" | "Owner dropped = memory freed" |
| "Runtime checks" | "Compile time guarantees" |
| "Duck typing" | "Explicit types" |

## Learning Resources

### For Python Developers

- [Rust for Python Programmers](https://lucumr.pocoo.org/2015/5/27/rust-for-pythonistas/)
- Compare with this project's `pure_python.py` and `lib.rs`

### For JavaScript Developers

- [Rust for JavaScript Developers](https://github.com/a-b-r-o-w-n/rust-for-javascript-developers)
- See [LANGUAGE_COMPARISON.md](LANGUAGE_COMPARISON.md) for detailed comparison

### General

- [The Rust Book](https://doc.rust-lang.org/book/) - Official guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [PyO3 Guide](https://pyo3.rs/) - Python integration

## Summary

**Key Rust concepts:**

1. **Ownership** - Every value has one owner
2. **Borrowing** - Can borrow with `&` (read) or `&mut` (write)
3. **Lifetimes** - Compiler ensures references are valid
4. **Type safety** - Catch errors at compile time
5. **Result/Option** - Explicit error and null handling
6. **Pattern matching** - Exhaustive case handling

**The payoff:**
- No null pointer errors
- No data races
- No memory leaks
- 20-50x performance improvement
- All checked at compile time with zero runtime cost!

**Next steps:**
- Read actual code: `rust_processor/src/lib.rs`
- Compare: `pure_python.py` vs `lib.rs`
- Learn more: [LANGUAGE_COMPARISON.md](LANGUAGE_COMPARISON.md)

---

**Ready to write Rust?** → Start with `rust_processor/src/lib.rs`

**Want deeper comparison?** → [LANGUAGE_COMPARISON.md](LANGUAGE_COMPARISON.md)

**Need navigation help?** → [NAVIGATION.md](NAVIGATION.md)
