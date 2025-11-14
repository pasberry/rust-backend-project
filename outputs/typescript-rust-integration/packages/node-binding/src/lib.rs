/*!
 * Node.js Bindings using napi-rs
 *
 * This module exposes the Rust core functionality to Node.js.
 * napi-rs automatically generates TypeScript type definitions!
 */

#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rust_core::{self, DataRecord as CoreDataRecord, ProcessResult as CoreProcessResult};
use std::collections::HashMap;

/// A single data record
///
/// This is exposed to TypeScript with automatic type generation
#[napi(object)]
#[derive(Debug, Clone)]
pub struct DataRecord {
    pub id: String,
    pub value: f64,
    pub category: String,
    pub timestamp: String,
    pub metadata: Option<HashMap<String, String>>,
}

impl From<DataRecord> for CoreDataRecord {
    fn from(record: DataRecord) -> Self {
        CoreDataRecord {
            id: record.id,
            value: record.value,
            category: record.category,
            timestamp: record.timestamp,
            metadata: record.metadata,
        }
    }
}

impl From<CoreDataRecord> for DataRecord {
    fn from(record: CoreDataRecord) -> Self {
        DataRecord {
            id: record.id,
            value: record.value,
            category: record.category,
            timestamp: record.timestamp,
            metadata: record.metadata,
        }
    }
}

/// Result of processing records
#[napi(object)]
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub total_processed: u32,
    pub total_value: f64,
    pub average_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub categories: HashMap<String, u32>,
}

impl From<CoreProcessResult> for ProcessResult {
    fn from(result: CoreProcessResult) -> Self {
        ProcessResult {
            total_processed: result.total_processed as u32,
            total_value: result.total_value,
            average_value: result.average_value,
            min_value: result.min_value,
            max_value: result.max_value,
            categories: result
                .categories
                .into_iter()
                .map(|(k, v)| (k, v as u32))
                .collect(),
        }
    }
}

/// Category statistics
#[napi(object)]
#[derive(Debug, Clone)]
pub struct CategoryStats {
    pub category: String,
    pub count: u32,
    pub total_value: f64,
    pub average_value: f64,
    pub min_value: f64,
    pub max_value: f64,
}

/// Validate a single record
///
/// Returns an error message if validation fails, or null if valid.
///
/// # Example (TypeScript)
/// ```typescript
/// const record = { id: "1", value: 100, category: "A", timestamp: "2024-01-15T10:00:00Z" };
/// const error = validateRecord(record);
/// if (error) {
///   console.error("Validation failed:", error);
/// }
/// ```
#[napi]
pub fn validate_record(record: DataRecord) -> Option<String> {
    let core_record: CoreDataRecord = record.into();
    match rust_core::validate_record(&core_record) {
        Ok(_) => None,
        Err(e) => Some(e.message),
    }
}

/// Process a batch of records and compute statistics
///
/// This is the main performance showcase - processes records in parallel using Rust.
///
/// # Example (TypeScript)
/// ```typescript
/// const records = [
///   { id: "1", value: 100, category: "A", timestamp: "2024-01-15T10:00:00Z" },
///   { id: "2", value: 200, category: "B", timestamp: "2024-01-15T10:00:01Z" },
/// ];
///
/// const result = processRecords(records);
/// console.log(`Processed ${result.total_processed} records`);
/// console.log(`Average value: ${result.average_value}`);
/// ```
#[napi]
pub fn process_records(records: Vec<DataRecord>) -> Result<ProcessResult> {
    let core_records: Vec<CoreDataRecord> = records.into_iter().map(|r| r.into()).collect();

    rust_core::process_records(&core_records)
        .map(|result| result.into())
        .map_err(|e| Error::new(Status::InvalidArg, e))
}

/// Filter records by category
///
/// Returns all records matching the specified category.
///
/// # Example (TypeScript)
/// ```typescript
/// const filtered = filterByCategory(records, "A");
/// console.log(`Found ${filtered.length} records in category A`);
/// ```
#[napi]
pub fn filter_by_category(records: Vec<DataRecord>, category: String) -> Vec<DataRecord> {
    let core_records: Vec<CoreDataRecord> = records.into_iter().map(|r| r.into()).collect();

    rust_core::filter_by_category(&core_records, &category)
        .into_iter()
        .map(|r| r.into())
        .collect()
}

/// Filter records by minimum value
///
/// Returns all records with value >= min_value.
///
/// # Example (TypeScript)
/// ```typescript
/// const expensive = filterByValue(records, 1000.0);
/// console.log(`Found ${expensive.length} records with value >= 1000`);
/// ```
#[napi]
pub fn filter_by_value(records: Vec<DataRecord>, min_value: f64) -> Vec<DataRecord> {
    let core_records: Vec<CoreDataRecord> = records.into_iter().map(|r| r.into()).collect();

    rust_core::filter_by_value(&core_records, min_value)
        .into_iter()
        .map(|r| r.into())
        .collect()
}

/// Get statistics for a specific category
///
/// Returns null if the category doesn't exist.
///
/// # Example (TypeScript)
/// ```typescript
/// const stats = getCategoryStats(records, "A");
/// if (stats) {
///   console.log(`Category A: ${stats.count} records, avg ${stats.average_value}`);
/// }
/// ```
#[napi]
pub fn get_category_stats(records: Vec<DataRecord>, category: String) -> Option<CategoryStats> {
    let core_records: Vec<CoreDataRecord> = records.into_iter().map(|r| r.into()).collect();

    rust_core::get_category_stats(&core_records, &category).map(|stats| CategoryStats {
        category: stats.category,
        count: stats.count as u32,
        total_value: stats.total_value,
        average_value: stats.average_value,
        min_value: stats.min_value,
        max_value: stats.max_value,
    })
}

/// Get all unique categories
///
/// Returns a sorted list of category names.
///
/// # Example (TypeScript)
/// ```typescript
/// const categories = getUniqueCategories(records);
/// console.log(`Found categories: ${categories.join(", ")}`);
/// ```
#[napi]
pub fn get_unique_categories(records: Vec<DataRecord>) -> Vec<String> {
    let core_records: Vec<CoreDataRecord> = records.into_iter().map(|r| r.into()).collect();
    rust_core::get_unique_categories(&core_records)
}

/// Benchmark helper: Process records and return processing time in milliseconds
///
/// This is useful for benchmarking to compare Rust vs pure JavaScript performance.
///
/// # Example (TypeScript)
/// ```typescript
/// const result = benchmarkProcess(records);
/// console.log(`Processed in ${result.duration_ms}ms`);
/// console.log(`Throughput: ${result.records_per_second} records/sec`);
/// ```
#[napi(object)]
pub struct BenchmarkResult {
    pub result: ProcessResult,
    pub duration_ms: f64,
    pub records_per_second: f64,
}

#[napi]
pub fn benchmark_process(records: Vec<DataRecord>) -> Result<BenchmarkResult> {
    use std::time::Instant;

    let core_records: Vec<CoreDataRecord> = records.into_iter().map(|r| r.into()).collect();

    let start = Instant::now();
    let result = rust_core::process_records(&core_records)
        .map_err(|e| Error::new(Status::InvalidArg, e))?;
    let duration = start.elapsed();

    let duration_ms = duration.as_secs_f64() * 1000.0;
    let records_per_second = result.total_processed as f64 / duration.as_secs_f64();

    Ok(BenchmarkResult {
        result: result.into(),
        duration_ms,
        records_per_second,
    })
}

/// Generate sample test data
///
/// Creates a specified number of random data records for testing.
///
/// # Example (TypeScript)
/// ```typescript
/// const testData = generateSampleData(1000);
/// console.log(`Generated ${testData.length} test records`);
/// ```
#[napi]
pub fn generate_sample_data(count: u32) -> Vec<DataRecord> {
    use std::collections::HashMap;

    let categories = vec!["A", "B", "C", "D"];
    let mut records = Vec::with_capacity(count as usize);

    for i in 0..count {
        let mut metadata = HashMap::new();
        metadata.insert("index".to_string(), i.to_string());

        records.push(DataRecord {
            id: format!("record_{}", i),
            value: ((i % 1000) as f64) * 10.5,
            category: categories[(i % 4) as usize].to_string(),
            timestamp: format!("2024-01-15T10:{:02}:{:02}Z", (i / 60) % 60, i % 60),
            metadata: Some(metadata),
        });
    }

    records
}
