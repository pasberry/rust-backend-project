/*!
 * WebAssembly Bindings using wasm-bindgen
 *
 * This module exposes the Rust core functionality to browsers via WASM.
 * Uses JSON for data transfer between JavaScript and Rust.
 */

use wasm_bindgen::prelude::*;
use rust_core::{self, DataRecord, ProcessResult as CoreProcessResult};

// Use web-sys for browser APIs
use web_sys::console;

/// Initialize WASM module
///
/// Call this before using any other functions.
///
/// # Example (JavaScript)
/// ```javascript
/// import init from './pkg/wasm_binding.js';
/// await init();
/// ```
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    console::log_1(&"Rust WASM module initialized!".into());
}

/// Log a message to the browser console (for debugging)
#[wasm_bindgen]
pub fn log(message: &str) {
    console::log_1(&message.into());
}

/// Process result with serialization helpers
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ProcessResult {
    result: CoreProcessResult,
}

#[wasm_bindgen]
impl ProcessResult {
    /// Get total number of processed records
    #[wasm_bindgen(getter)]
    pub fn total_processed(&self) -> usize {
        self.result.total_processed
    }

    /// Get total value
    #[wasm_bindgen(getter)]
    pub fn total_value(&self) -> f64 {
        self.result.total_value
    }

    /// Get average value
    #[wasm_bindgen(getter)]
    pub fn average_value(&self) -> f64 {
        self.result.average_value
    }

    /// Get minimum value
    #[wasm_bindgen(getter)]
    pub fn min_value(&self) -> f64 {
        self.result.min_value
    }

    /// Get maximum value
    #[wasm_bindgen(getter)]
    pub fn max_value(&self) -> f64 {
        self.result.max_value
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.result)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// Validate a single record
///
/// Takes JSON string, returns error message or null if valid.
///
/// # Example (JavaScript)
/// ```javascript
/// const record = JSON.stringify({ id: "1", value: 100, category: "A", timestamp: "2024-01-15T10:00:00Z" });
/// const error = validateRecord(record);
/// if (error) {
///   console.error("Validation failed:", error);
/// }
/// ```
#[wasm_bindgen(js_name = validateRecord)]
pub fn validate_record(record_json: &str) -> Option<String> {
    let record: DataRecord = match serde_json::from_str(record_json) {
        Ok(r) => r,
        Err(e) => return Some(format!("JSON parse error: {}", e)),
    };

    match rust_core::validate_record(&record) {
        Ok(_) => None,
        Err(e) => Some(e.message),
    }
}

/// Process records and compute statistics
///
/// Takes JSON array of records, returns ProcessResult.
///
/// # Example (JavaScript)
/// ```javascript
/// const records = [
///   { id: "1", value: 100, category: "A", timestamp: "2024-01-15T10:00:00Z" },
///   { id: "2", value: 200, category: "B", timestamp: "2024-01-15T10:00:01Z" },
/// ];
///
/// const result = processRecords(JSON.stringify(records));
/// console.log(`Processed ${result.total_processed} records`);
/// console.log(`Average: ${result.average_value}`);
/// ```
#[wasm_bindgen(js_name = processRecords)]
pub fn process_records(records_json: &str) -> Result<ProcessResult, JsValue> {
    let records: Vec<DataRecord> = serde_json::from_str(records_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    let result = rust_core::process_records(&records)
        .map_err(|e| JsValue::from_str(&e))?;

    Ok(ProcessResult { result })
}

/// Filter records by category
///
/// Takes JSON array of records and category name, returns JSON array of filtered records.
///
/// # Example (JavaScript)
/// ```javascript
/// const filtered = filterByCategory(JSON.stringify(records), "A");
/// const results = JSON.parse(filtered);
/// console.log(`Found ${results.length} records in category A`);
/// ```
#[wasm_bindgen(js_name = filterByCategory)]
pub fn filter_by_category(records_json: &str, category: &str) -> Result<String, JsValue> {
    let records: Vec<DataRecord> = serde_json::from_str(records_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    let filtered = rust_core::filter_by_category(&records, category);

    serde_json::to_string(&filtered)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Filter records by minimum value
///
/// Takes JSON array of records and min_value, returns JSON array of filtered records.
///
/// # Example (JavaScript)
/// ```javascript
/// const filtered = filterByValue(JSON.stringify(records), 1000.0);
/// const results = JSON.parse(filtered);
/// console.log(`Found ${results.length} records with value >= 1000`);
/// ```
#[wasm_bindgen(js_name = filterByValue)]
pub fn filter_by_value(records_json: &str, min_value: f64) -> Result<String, JsValue> {
    let records: Vec<DataRecord> = serde_json::from_str(records_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    let filtered = rust_core::filter_by_value(&records, min_value);

    serde_json::to_string(&filtered)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get statistics for a specific category
///
/// Returns JSON string with stats, or null if category doesn't exist.
///
/// # Example (JavaScript)
/// ```javascript
/// const statsJson = getCategoryStats(JSON.stringify(records), "A");
/// if (statsJson) {
///   const stats = JSON.parse(statsJson);
///   console.log(`Category A: ${stats.count} records, avg ${stats.average_value}`);
/// }
/// ```
#[wasm_bindgen(js_name = getCategoryStats)]
pub fn get_category_stats(records_json: &str, category: &str) -> Result<Option<String>, JsValue> {
    let records: Vec<DataRecord> = serde_json::from_str(records_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    let stats = rust_core::get_category_stats(&records, category);

    match stats {
        Some(s) => {
            let json = serde_json::to_string(&s)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Some(json))
        }
        None => Ok(None),
    }
}

/// Get all unique categories
///
/// Returns JSON array of category names.
///
/// # Example (JavaScript)
/// ```javascript
/// const categoriesJson = getUniqueCategories(JSON.stringify(records));
/// const categories = JSON.parse(categoriesJson);
/// console.log(`Found categories: ${categories.join(", ")}`);
/// ```
#[wasm_bindgen(js_name = getUniqueCategories)]
pub fn get_unique_categories(records_json: &str) -> Result<String, JsValue> {
    let records: Vec<DataRecord> = serde_json::from_str(records_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    let categories = rust_core::get_unique_categories(&records);

    serde_json::to_string(&categories)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Benchmark: Process records and measure performance using browser APIs
///
/// Returns JSON with result and timing information.
///
/// # Example (JavaScript)
/// ```javascript
/// const result = benchmarkProcess(JSON.stringify(records));
/// const data = JSON.parse(result);
/// console.log(`Processed in ${data.duration_ms}ms`);
/// console.log(`Throughput: ${data.records_per_second} records/sec`);
/// ```
#[wasm_bindgen(js_name = benchmarkProcess)]
pub fn benchmark_process(records_json: &str) -> Result<String, JsValue> {
    // Use browser's performance API for accurate timing
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))?;
    let performance = window.performance().ok_or_else(|| JsValue::from_str("No performance object"))?;

    let records: Vec<DataRecord> = serde_json::from_str(records_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    let start = performance.now();
    let result = rust_core::process_records(&records)
        .map_err(|e| JsValue::from_str(&e))?;
    let end = performance.now();

    let duration_ms = end - start;
    let records_per_second = result.total_processed as f64 / (duration_ms / 1000.0);

    let benchmark_result = serde_json::json!({
        "result": result,
        "duration_ms": duration_ms,
        "records_per_second": records_per_second,
    });

    serde_json::to_string(&benchmark_result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Generate sample test data
///
/// Creates a specified number of random data records.
/// Returns JSON array.
///
/// # Example (JavaScript)
/// ```javascript
/// const testDataJson = generateSampleData(1000);
/// const testData = JSON.parse(testDataJson);
/// console.log(`Generated ${testData.length} test records`);
/// ```
#[wasm_bindgen(js_name = generateSampleData)]
pub fn generate_sample_data(count: usize) -> Result<String, JsValue> {
    let categories = vec!["A", "B", "C", "D"];
    let mut records = Vec::with_capacity(count);

    for i in 0..count {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("index".to_string(), i.to_string());

        records.push(DataRecord {
            id: format!("record_{}", i),
            value: ((i % 1000) as f64) * 10.5,
            category: categories[i % 4].to_string(),
            timestamp: format!("2024-01-15T10:{:02}:{:02}Z", (i / 60) % 60, i % 60),
            metadata: Some(metadata),
        });
    }

    serde_json::to_string(&records)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_validate_record() {
        let valid = r#"{"id":"1","value":100,"category":"A","timestamp":"2024-01-15T10:00:00Z"}"#;
        assert!(validate_record(valid).is_none());

        let invalid = r#"{"id":"","value":100,"category":"A","timestamp":"2024-01-15T10:00:00Z"}"#;
        assert!(validate_record(invalid).is_some());
    }

    #[wasm_bindgen_test]
    fn test_generate_sample_data() {
        let data_json = generate_sample_data(10).unwrap();
        let data: Vec<DataRecord> = serde_json::from_str(&data_json).unwrap();
        assert_eq!(data.len(), 10);
    }
}
