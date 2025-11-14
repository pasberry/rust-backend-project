/*!
 * Shared Rust Core Library
 *
 * This module contains platform-agnostic business logic shared between
 * Node.js (napi-rs) and WASM (wasm-bindgen) bindings.
 *
 * Key principle: Keep this pure Rust with no platform-specific code.
 */

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single data record for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    pub id: String,
    pub value: f64,
    pub category: String,
    pub timestamp: String,
    pub metadata: Option<HashMap<String, String>>,
}

/// Result of processing a batch of records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub total_processed: usize,
    pub total_value: f64,
    pub average_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub categories: HashMap<String, usize>,
}

/// Statistics for a specific category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category: String,
    pub count: usize,
    pub total_value: f64,
    pub average_value: f64,
    pub min_value: f64,
    pub max_value: f64,
}

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub record_id: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(record_id: String, message: String) -> Self {
        Self { record_id, message }
    }
}

/// Validate a single record
///
/// Checks:
/// - Value is positive
/// - Category is not empty
/// - ID is not empty
/// - Timestamp is not empty
pub fn validate_record(record: &DataRecord) -> Result<(), ValidationError> {
    if record.id.is_empty() {
        return Err(ValidationError::new(
            record.id.clone(),
            "ID cannot be empty".to_string(),
        ));
    }

    if record.value < 0.0 {
        return Err(ValidationError::new(
            record.id.clone(),
            format!("Value must be positive, got {}", record.value),
        ));
    }

    if record.category.is_empty() {
        return Err(ValidationError::new(
            record.id.clone(),
            "Category cannot be empty".to_string(),
        ));
    }

    if record.timestamp.is_empty() {
        return Err(ValidationError::new(
            record.id.clone(),
            "Timestamp cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Process a batch of records and compute statistics
///
/// This is the main computation function that demonstrates Rust's performance
/// advantages. It uses parallel processing to handle large datasets efficiently.
pub fn process_records(records: &[DataRecord]) -> Result<ProcessResult, String> {
    if records.is_empty() {
        return Err("Cannot process empty record set".to_string());
    }

    // Validate all records first
    let validation_errors: Vec<_> = records
        .par_iter()
        .filter_map(|record| validate_record(record).err())
        .collect();

    if !validation_errors.is_empty() {
        let error_msg = validation_errors
            .iter()
            .map(|e| format!("Record {}: {}", e.record_id, e.message))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(format!("Validation errors: {}", error_msg));
    }

    // Compute statistics in parallel
    let total_processed = records.len();

    let total_value: f64 = records.par_iter().map(|r| r.value).sum();

    let values: Vec<f64> = records.par_iter().map(|r| r.value).collect();
    let min_value = values
        .par_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap_or(0.0);
    let max_value = values
        .par_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap_or(0.0);

    let average_value = total_value / total_processed as f64;

    // Count by category
    let mut categories: HashMap<String, usize> = HashMap::new();
    for record in records {
        *categories.entry(record.category.clone()).or_insert(0) += 1;
    }

    Ok(ProcessResult {
        total_processed,
        total_value,
        average_value,
        min_value,
        max_value,
        categories,
    })
}

/// Filter records by category
///
/// Returns all records matching the specified category.
/// Uses parallel iteration for performance on large datasets.
pub fn filter_by_category(records: &[DataRecord], category: &str) -> Vec<DataRecord> {
    records
        .par_iter()
        .filter(|record| record.category == category)
        .cloned()
        .collect()
}

/// Filter records by value threshold
///
/// Returns all records with value >= min_value.
pub fn filter_by_value(records: &[DataRecord], min_value: f64) -> Vec<DataRecord> {
    records
        .par_iter()
        .filter(|record| record.value >= min_value)
        .cloned()
        .collect()
}

/// Get statistics for a specific category
pub fn get_category_stats(records: &[DataRecord], category: &str) -> Option<CategoryStats> {
    let filtered: Vec<&DataRecord> = records
        .par_iter()
        .filter(|r| r.category == category)
        .collect();

    if filtered.is_empty() {
        return None;
    }

    let count = filtered.len();
    let values: Vec<f64> = filtered.iter().map(|r| r.value).collect();

    let total_value: f64 = values.iter().sum();
    let average_value = total_value / count as f64;

    let min_value = values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap_or(0.0);
    let max_value = values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap_or(0.0);

    Some(CategoryStats {
        category: category.to_string(),
        count,
        total_value,
        average_value,
        min_value,
        max_value,
    })
}

/// Transform records: apply a multiplier to all values
///
/// This demonstrates a common transformation pattern.
pub fn transform_values(records: &mut [DataRecord], multiplier: f64) {
    records.par_iter_mut().for_each(|record| {
        record.value *= multiplier;
    });
}

/// Get unique categories from records
pub fn get_unique_categories(records: &[DataRecord]) -> Vec<String> {
    let mut categories: Vec<String> = records
        .iter()
        .map(|r| r.category.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    categories.sort();
    categories
}

/// Aggregate records by category, returning stats for each
pub fn aggregate_by_category(records: &[DataRecord]) -> HashMap<String, CategoryStats> {
    let categories = get_unique_categories(records);

    categories
        .into_iter()
        .filter_map(|cat| get_category_stats(records, &cat).map(|stats| (cat, stats)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_record(id: &str, value: f64, category: &str) -> DataRecord {
        DataRecord {
            id: id.to_string(),
            value,
            category: category.to_string(),
            timestamp: "2024-01-15T10:00:00Z".to_string(),
            metadata: None,
        }
    }

    #[test]
    fn test_validate_record() {
        let valid = create_test_record("1", 100.0, "A");
        assert!(validate_record(&valid).is_ok());

        let invalid_empty_id = DataRecord {
            id: "".to_string(),
            ..valid.clone()
        };
        assert!(validate_record(&invalid_empty_id).is_err());

        let invalid_negative = DataRecord {
            value: -10.0,
            ..valid.clone()
        };
        assert!(validate_record(&invalid_negative).is_err());
    }

    #[test]
    fn test_process_records() {
        let records = vec![
            create_test_record("1", 100.0, "A"),
            create_test_record("2", 200.0, "B"),
            create_test_record("3", 150.0, "A"),
        ];

        let result = process_records(&records).unwrap();
        assert_eq!(result.total_processed, 3);
        assert_eq!(result.total_value, 450.0);
        assert_eq!(result.average_value, 150.0);
        assert_eq!(result.min_value, 100.0);
        assert_eq!(result.max_value, 200.0);
    }

    #[test]
    fn test_filter_by_category() {
        let records = vec![
            create_test_record("1", 100.0, "A"),
            create_test_record("2", 200.0, "B"),
            create_test_record("3", 150.0, "A"),
        ];

        let filtered = filter_by_category(&records, "A");
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_get_category_stats() {
        let records = vec![
            create_test_record("1", 100.0, "A"),
            create_test_record("2", 200.0, "A"),
            create_test_record("3", 150.0, "B"),
        ];

        let stats = get_category_stats(&records, "A").unwrap();
        assert_eq!(stats.count, 2);
        assert_eq!(stats.total_value, 300.0);
        assert_eq!(stats.average_value, 150.0);
    }
}
