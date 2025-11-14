/**
 * Node.js Example: Using Rust via napi-rs
 *
 * This demonstrates 5 scenarios using the Rust-powered Node.js binding
 */

import * as rust from '@rust-integration/node-binding';
import { DataRecord } from './types';

function printHeader(title: string) {
    console.log('\n' + '='.repeat(70));
    console.log(`  ${title}`);
    console.log('='.repeat(70) + '\n');
}

/**
 * Example 1: Basic Processing
 */
function example1_basicProcessing() {
    printHeader('Example 1: Basic Processing');

    const records: DataRecord[] = [
        { id: '1', value: 100, category: 'A', timestamp: '2024-01-15T10:00:00Z' },
        { id: '2', value: 200, category: 'B', timestamp: '2024-01-15T10:00:01Z' },
        { id: '3', value: 150, category: 'A', timestamp: '2024-01-15T10:00:02Z' },
    ];

    console.log(`Processing ${records.length} records...`);

    const result = rust.processRecords(records);

    console.log(`✅ Results:`);
    console.log(`   Total processed: ${result.total_processed}`);
    console.log(`   Average value: ${result.average_value.toFixed(2)}`);
    console.log(`   Min value: ${result.min_value}`);
    console.log(`   Max value: ${result.max_value}`);
    console.log(`   Categories:`, result.categories);
}

/**
 * Example 2: Validation
 */
function example2_validation() {
    printHeader('Example 2: Data Validation');

    const validRecord: DataRecord = {
        id: '1',
        value: 100,
        category: 'A',
        timestamp: '2024-01-15T10:00:00Z',
    };

    const invalidRecords = [
        { id: '', value: 100, category: 'A', timestamp: '2024-01-15T10:00:00Z' },
        { id: '2', value: -50, category: 'B', timestamp: '2024-01-15T10:00:01Z' },
        { id: '3', value: 100, category: '', timestamp: '2024-01-15T10:00:02Z' },
    ];

    console.log('Testing valid record:');
    const validError = rust.validateRecord(validRecord);
    console.log(validError ? `  ❌ ${validError}` : '  ✅ Valid');

    console.log('\nTesting invalid records:');
    invalidRecords.forEach((record, i) => {
        const error = rust.validateRecord(record as DataRecord);
        console.log(`  Record ${i + 1}: ${error ? `❌ ${error}` : '✅ Valid'}`);
    });
}

/**
 * Example 3: Filtering
 */
function example3_filtering() {
    printHeader('Example 3: Filtering Data');

    const records = rust.generateSampleData(100);
    console.log(`Generated ${records.length} sample records\n`);

    // Filter by category
    const categoryA = rust.filterByCategory(records, 'A');
    console.log(`Category A: ${categoryA.length} records`);

    const categoryB = rust.filterByCategory(records, 'B');
    console.log(`Category B: ${categoryB.length} records`);

    // Filter by value
    const highValue = rust.filterByValue(records, 5000);
    console.log(`\nHigh value (>= 5000): ${highValue.length} records`);

    const lowValue = rust.filterByValue(records, 1000);
    console.log(`Medium value (>= 1000): ${lowValue.length} records`);
}

/**
 * Example 4: Category Statistics
 */
function example4_categoryStats() {
    printHeader('Example 4: Category Statistics');

    const records = rust.generateSampleData(1000);
    console.log(`Analyzing ${records.length} records...\n`);

    const categories = rust.getUniqueCategories(records);
    console.log(`Found ${categories.length} categories: ${categories.join(', ')}\n`);

    categories.forEach(category => {
        const stats = rust.getCategoryStats(records, category);
        if (stats) {
            console.log(`Category ${category}:`);
            console.log(`  Count: ${stats.count}`);
            console.log(`  Total value: ${stats.total_value.toFixed(2)}`);
            console.log(`  Average: ${stats.average_value.toFixed(2)}`);
            console.log(`  Range: ${stats.min_value} - ${stats.max_value}`);
            console.log();
        }
    });
}

/**
 * Example 5: Performance Benchmark
 */
function example5_benchmark() {
    printHeader('Example 5: Performance Benchmark');

    const testSizes = [1_000, 10_000, 100_000];

    console.log('Running benchmarks with different dataset sizes...\n');

    testSizes.forEach(size => {
        const records = rust.generateSampleData(size);

        const result = rust.benchmarkProcess(records);

        console.log(`Dataset: ${size.toLocaleString()} records`);
        console.log(`  Duration: ${result.duration_ms.toFixed(2)}ms`);
        console.log(`  Throughput: ${result.records_per_second.toLocaleString(undefined, { maximumFractionDigits: 0 })} records/sec`);
        console.log(`  Processed: ${result.result.total_processed} records`);
        console.log();
    });
}

/**
 * Main function
 */
function main() {
    console.log('\n' + '='.repeat(70));
    console.log('  Rust + Node.js Integration Examples');
    console.log('='.repeat(70));
    console.log('\nDemonstrating Rust-powered data processing in Node.js');
    console.log('using napi-rs for native bindings.\n');

    try {
        example1_basicProcessing();
        example2_validation();
        example3_filtering();
        example4_categoryStats();
        example5_benchmark();

        console.log('='.repeat(70));
        console.log('  ✅ All examples completed successfully!');
        console.log('='.repeat(70));
        console.log('\nNext steps:');
        console.log('  • Run benchmarks: npm run benchmark');
        console.log('  • Check browser demo: open public/index.html');
        console.log('  • Read documentation in docs/');
        console.log('='.repeat(70) + '\n');
    } catch (error) {
        console.error('\n❌ Error:', error);
        process.exit(1);
    }
}

// Run if executed directly
if (require.main === module) {
    main();
}
