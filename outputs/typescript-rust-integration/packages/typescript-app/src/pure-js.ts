/**
 * Pure JavaScript Implementation (for benchmarking)
 *
 * This provides JavaScript-only implementations to compare against Rust performance.
 */

import { DataRecord, ProcessResult, CategoryStats } from './types';

export class PureJSProcessor {
    static validateRecord(record: DataRecord): string | null {
        if (!record.id || record.id.length === 0) {
            return 'ID cannot be empty';
        }
        if (record.value < 0) {
            return `Value must be positive, got ${record.value}`;
        }
        if (!record.category || record.category.length === 0) {
            return 'Category cannot be empty';
        }
        if (!record.timestamp || record.timestamp.length === 0) {
            return 'Timestamp cannot be empty';
        }
        return null;
    }

    static processRecords(records: DataRecord[]): ProcessResult {
        if (records.length === 0) {
            throw new Error('Cannot process empty record set');
        }

        // Validate all
        for (const record of records) {
            const error = this.validateRecord(record);
            if (error) {
                throw new Error(`Validation error for record ${record.id}: ${error}`);
            }
        }

        const values = records.map(r => r.value);
        const totalValue = values.reduce((a, b) => a + b, 0);

        const categories: Record<string, number> = {};
        for (const record of records) {
            categories[record.category] = (categories[record.category] || 0) + 1;
        }

        return {
            total_processed: records.length,
            total_value: totalValue,
            average_value: totalValue / records.length,
            min_value: Math.min(...values),
            max_value: Math.max(...values),
            categories,
        };
    }

    static filterByCategory(records: DataRecord[], category: string): DataRecord[] {
        return records.filter(r => r.category === category);
    }

    static filterByValue(records: DataRecord[], minValue: number): DataRecord[] {
        return records.filter(r => r.value >= minValue);
    }

    static getCategoryStats(records: DataRecord[], category: string): CategoryStats | null {
        const filtered = records.filter(r => r.category === category);
        if (filtered.length === 0) {
            return null;
        }

        const values = filtered.map(r => r.value);
        const totalValue = values.reduce((a, b) => a + b, 0);

        return {
            category,
            count: filtered.length,
            total_value: totalValue,
            average_value: totalValue / filtered.length,
            min_value: Math.min(...values),
            max_value: Math.max(...values),
        };
    }

    static getUniqueCategories(records: DataRecord[]): string[] {
        const categories = new Set(records.map(r => r.category));
        return Array.from(categories).sort();
    }
}
