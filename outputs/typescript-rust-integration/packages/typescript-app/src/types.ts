/**
 * Shared TypeScript Type Definitions
 *
 * These types are used across both Node.js and browser environments.
 */

export interface DataRecord {
    id: string;
    value: number;
    category: string;
    timestamp: string;
    metadata?: Record<string, string>;
}

export interface ProcessResult {
    total_processed: number;
    total_value: number;
    average_value: number;
    min_value: number;
    max_value: number;
    categories: Record<string, number>;
}

export interface CategoryStats {
    category: string;
    count: number;
    total_value: number;
    average_value: number;
    min_value: number;
    max_value: number;
}

export interface BenchmarkResult {
    result: ProcessResult;
    duration_ms: number;
    records_per_second: number;
}

/**
 * Generate sample test data
 */
export function generateSampleData(count: number): DataRecord[] {
    const categories = ['A', 'B', 'C', 'D'];
    const records: DataRecord[] = [];

    for (let i = 0; i < count; i++) {
        records.push({
            id: `record_${i}`,
            value: (i % 1000) * 10.5,
            category: categories[i % 4],
            timestamp: `2024-01-15T10:${Math.floor((i / 60) % 60).toString().padStart(2, '0')}:${(i % 60).toString().padStart(2, '0')}Z`,
            metadata: {
                index: i.toString(),
            },
        });
    }

    return records;
}
