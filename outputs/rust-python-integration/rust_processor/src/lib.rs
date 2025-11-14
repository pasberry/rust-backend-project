use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single log entry after parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub duration_ms: Option<f64>,
    pub status_code: Option<i32>,
    pub user_id: Option<String>,
}

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub line_number: usize,
    pub message: String,
}

/// Statistics computed from log entries
/// This is exposed to Python as a class with accessible properties
#[pyclass]
#[derive(Debug, Clone)]
pub struct LogStats {
    #[pyo3(get)]
    pub total_count: usize,

    #[pyo3(get)]
    pub error_count: usize,

    #[pyo3(get)]
    pub warn_count: usize,

    #[pyo3(get)]
    pub info_count: usize,

    #[pyo3(get)]
    pub avg_duration_ms: f64,

    #[pyo3(get)]
    pub min_duration_ms: f64,

    #[pyo3(get)]
    pub max_duration_ms: f64,

    #[pyo3(get)]
    pub p50_duration_ms: f64,

    #[pyo3(get)]
    pub p95_duration_ms: f64,

    #[pyo3(get)]
    pub p99_duration_ms: f64,

    #[pyo3(get)]
    pub status_code_distribution: HashMap<i32, usize>,

    #[pyo3(get)]
    pub error_count_by_code: HashMap<i32, usize>,
}

#[pymethods]
impl LogStats {
    /// String representation for Python
    fn __repr__(&self) -> String {
        format!(
            "LogStats(total={}, errors={}, avg_duration={:.2}ms, p95={:.2}ms)",
            self.total_count, self.error_count, self.avg_duration_ms, self.p95_duration_ms
        )
    }

    /// Get a summary string
    fn summary(&self) -> String {
        format!(
            "Total logs: {}\n\
             Error count: {}\n\
             Warning count: {}\n\
             Info count: {}\n\
             Average duration: {:.2}ms\n\
             P95 duration: {:.2}ms\n\
             P99 duration: {:.2}ms",
            self.total_count,
            self.error_count,
            self.warn_count,
            self.info_count,
            self.avg_duration_ms,
            self.p95_duration_ms,
            self.p99_duration_ms
        )
    }
}

/// Parse JSON log strings in parallel
///
/// This function demonstrates Pattern 2: offloading CPU-intensive parsing to Rust
/// while Python handles I/O and orchestration. Using Rayon for parallel processing
/// provides significant speedup over pure Python parsing.
///
/// # Arguments
/// * `log_lines` - Vector of JSON strings, one per log entry
///
/// # Returns
/// * Result containing vector of parsed LogEntry objects or error message
#[pyfunction]
fn parse_logs(log_lines: Vec<String>) -> PyResult<Vec<HashMap<String, String>>> {
    // Use Rayon to parse logs in parallel across available CPU cores
    // This is where we get the major performance win - Python's GIL doesn't apply here!
    let results: Result<Vec<LogEntry>, _> = log_lines
        .par_iter()
        .map(|line| {
            serde_json::from_str::<LogEntry>(line)
                .map_err(|e| format!("Parse error: {}", e))
        })
        .collect();

    match results {
        Ok(entries) => {
            // Convert to Python-friendly format (HashMap)
            let py_entries: Vec<HashMap<String, String>> = entries
                .iter()
                .map(|entry| {
                    let mut map = HashMap::new();
                    map.insert("timestamp".to_string(), entry.timestamp.clone());
                    map.insert("level".to_string(), entry.level.clone());
                    map.insert("message".to_string(), entry.message.clone());
                    if let Some(duration) = entry.duration_ms {
                        map.insert("duration_ms".to_string(), duration.to_string());
                    }
                    if let Some(status) = entry.status_code {
                        map.insert("status_code".to_string(), status.to_string());
                    }
                    if let Some(ref user_id) = entry.user_id {
                        map.insert("user_id".to_string(), user_id.clone());
                    }
                    map
                })
                .collect();
            Ok(py_entries)
        }
        Err(e) => Err(PyValueError::new_err(e)),
    }
}

/// Validate log entries with detailed error reporting
///
/// This function checks log schema and returns detailed validation errors.
/// The validation rules demonstrate how Rust's type system and error handling
/// can catch issues that might slip through in dynamically-typed Python.
///
/// # Arguments
/// * `log_lines` - Vector of JSON strings to validate
///
/// # Returns
/// * Tuple of (valid_count, error_messages)
#[pyfunction]
fn validate_logs(log_lines: Vec<String>) -> PyResult<(usize, Vec<String>)> {
    let results: Vec<Result<LogEntry, String>> = log_lines
        .par_iter()
        .enumerate()
        .map(|(idx, line)| {
            // Try to parse
            let entry: LogEntry = serde_json::from_str(line)
                .map_err(|e| format!("Line {}: JSON parse error: {}", idx + 1, e))?;

            // Validate required fields
            if entry.timestamp.is_empty() {
                return Err(format!("Line {}: Missing or empty timestamp", idx + 1));
            }

            // Validate log level
            let valid_levels = ["ERROR", "WARN", "INFO", "DEBUG"];
            if !valid_levels.contains(&entry.level.as_str()) {
                return Err(format!(
                    "Line {}: Invalid log level '{}'. Must be one of: ERROR, WARN, INFO, DEBUG",
                    idx + 1,
                    entry.level
                ));
            }

            // Validate duration if present
            if let Some(duration) = entry.duration_ms {
                if duration < 0.0 {
                    return Err(format!(
                        "Line {}: Invalid duration_ms {}. Must be >= 0",
                        idx + 1,
                        duration
                    ));
                }
            }

            // Validate status code if present
            if let Some(status) = entry.status_code {
                if !(100..=599).contains(&status) {
                    return Err(format!(
                        "Line {}: Invalid status_code {}. Must be 100-599",
                        idx + 1,
                        status
                    ));
                }
            }

            Ok(entry)
        })
        .collect();

    let mut errors = Vec::new();
    let mut valid_count = 0;

    for result in results {
        match result {
            Ok(_) => valid_count += 1,
            Err(e) => errors.push(e),
        }
    }

    Ok((valid_count, errors))
}

/// Compute comprehensive statistics from log entries
///
/// This is the performance showcase function - it processes potentially millions
/// of log entries to compute aggregations and percentiles in parallel. This would
/// be 20-50x slower in pure Python due to GIL and interpreted nature.
///
/// # Arguments
/// * `log_lines` - Vector of JSON log strings
///
/// # Returns
/// * LogStats object with all computed statistics
#[pyfunction]
fn compute_stats(log_lines: Vec<String>) -> PyResult<LogStats> {
    // Parse all logs in parallel
    let entries: Vec<LogEntry> = log_lines
        .par_iter()
        .filter_map(|line| serde_json::from_str::<LogEntry>(line).ok())
        .collect();

    if entries.is_empty() {
        return Err(PyValueError::new_err("No valid log entries found"));
    }

    // Count by log level
    let error_count = entries.par_iter().filter(|e| e.level == "ERROR").count();
    let warn_count = entries.par_iter().filter(|e| e.level == "WARN").count();
    let info_count = entries.par_iter().filter(|e| e.level == "INFO").count();

    // Collect all durations for percentile calculation
    let mut durations: Vec<f64> = entries
        .par_iter()
        .filter_map(|e| e.duration_ms)
        .collect();

    let (avg_duration, min_duration, max_duration, p50, p95, p99) = if durations.is_empty() {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    } else {
        // Sort for percentile calculation
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let sum: f64 = durations.iter().sum();
        let avg = sum / durations.len() as f64;
        let min = durations[0];
        let max = durations[durations.len() - 1];

        // Calculate percentiles
        let p50_idx = (durations.len() as f64 * 0.50) as usize;
        let p95_idx = (durations.len() as f64 * 0.95) as usize;
        let p99_idx = (durations.len() as f64 * 0.99) as usize;

        let p50 = durations[p50_idx.min(durations.len() - 1)];
        let p95 = durations[p95_idx.min(durations.len() - 1)];
        let p99 = durations[p99_idx.min(durations.len() - 1)];

        (avg, min, max, p50, p95, p99)
    };

    // Status code distribution
    let mut status_code_distribution = HashMap::new();
    for entry in &entries {
        if let Some(code) = entry.status_code {
            *status_code_distribution.entry(code).or_insert(0) += 1;
        }
    }

    // Error codes (4xx, 5xx)
    let mut error_count_by_code = HashMap::new();
    for entry in &entries {
        if let Some(code) = entry.status_code {
            if code >= 400 {
                *error_count_by_code.entry(code).or_insert(0) += 1;
            }
        }
    }

    Ok(LogStats {
        total_count: entries.len(),
        error_count,
        warn_count,
        info_count,
        avg_duration_ms: avg_duration,
        min_duration_ms: min_duration,
        max_duration_ms: max_duration,
        p50_duration_ms: p50,
        p95_duration_ms: p95,
        p99_duration_ms: p99,
        status_code_distribution,
        error_count_by_code,
    })
}

/// Filter logs by various criteria
///
/// This function demonstrates complex filtering logic that benefits from Rust's
/// performance. Multiple criteria can be applied simultaneously, and the parallel
/// processing ensures fast results even with large datasets.
///
/// # Arguments
/// * `log_lines` - Vector of JSON log strings
/// * `min_level` - Minimum log level (ERROR=3, WARN=2, INFO=1, DEBUG=0)
/// * `min_duration_ms` - Minimum duration in milliseconds (None = no filter)
/// * `status_codes` - List of status codes to include (empty = all)
///
/// # Returns
/// * Filtered list of log entries as HashMaps
#[pyfunction]
fn filter_logs(
    log_lines: Vec<String>,
    min_level: Option<String>,
    min_duration_ms: Option<f64>,
    status_codes: Option<Vec<i32>>,
) -> PyResult<Vec<HashMap<String, String>>> {
    // Parse all logs in parallel
    let entries: Vec<LogEntry> = log_lines
        .par_iter()
        .filter_map(|line| serde_json::from_str::<LogEntry>(line).ok())
        .collect();

    // Helper to convert level to numeric value for comparison
    let level_to_num = |level: &str| -> i32 {
        match level {
            "ERROR" => 3,
            "WARN" => 2,
            "INFO" => 1,
            "DEBUG" => 0,
            _ => 0,
        }
    };

    let min_level_num = min_level
        .as_ref()
        .map(|l| level_to_num(l))
        .unwrap_or(0);

    // Apply filters in parallel
    let filtered: Vec<LogEntry> = entries
        .into_par_iter()
        .filter(|entry| {
            // Check log level
            if level_to_num(&entry.level) < min_level_num {
                return false;
            }

            // Check duration
            if let Some(min_dur) = min_duration_ms {
                if let Some(dur) = entry.duration_ms {
                    if dur < min_dur {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Check status codes
            if let Some(ref codes) = status_codes {
                if !codes.is_empty() {
                    if let Some(code) = entry.status_code {
                        if !codes.contains(&code) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }

            true
        })
        .collect();

    // Convert to Python-friendly format
    let result: Vec<HashMap<String, String>> = filtered
        .iter()
        .map(|entry| {
            let mut map = HashMap::new();
            map.insert("timestamp".to_string(), entry.timestamp.clone());
            map.insert("level".to_string(), entry.level.clone());
            map.insert("message".to_string(), entry.message.clone());
            if let Some(duration) = entry.duration_ms {
                map.insert("duration_ms".to_string(), duration.to_string());
            }
            if let Some(status) = entry.status_code {
                map.insert("status_code".to_string(), status.to_string());
            }
            if let Some(ref user_id) = entry.user_id {
                map.insert("user_id".to_string(), user_id.clone());
            }
            map
        })
        .collect();

    Ok(result)
}

/// Batch process logs with all operations
///
/// This is a convenience function that combines parsing, validation, and stats
/// in a single call, reducing the overhead of crossing the Python-Rust boundary
/// multiple times. This pattern is recommended for production use.
///
/// # Arguments
/// * `log_lines` - Vector of JSON log strings
///
/// # Returns
/// * Tuple of (LogStats, error_messages)
#[pyfunction]
fn batch_process(log_lines: Vec<String>) -> PyResult<(LogStats, Vec<String>)> {
    let (_, errors) = validate_logs(log_lines.clone())?;
    let stats = compute_stats(log_lines)?;
    Ok((stats, errors))
}

/// Python module definition
///
/// This is where we expose our Rust functions to Python. PyO3 handles all the
/// type conversions and memory management automatically. The module can be
/// imported in Python as: `import rust_processor`
#[pymodule]
fn rust_processor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_logs, m)?)?;
    m.add_function(wrap_pyfunction!(validate_logs, m)?)?;
    m.add_function(wrap_pyfunction!(compute_stats, m)?)?;
    m.add_function(wrap_pyfunction!(filter_logs, m)?)?;
    m.add_function(wrap_pyfunction!(batch_process, m)?)?;
    m.add_class::<LogStats>()?;
    Ok(())
}
