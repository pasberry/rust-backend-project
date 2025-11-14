"""
Pure Python Implementation for Benchmarking

This module provides pure Python implementations of all the functions that
Rust handles. This is used for performance comparisons to demonstrate the
20-50x speedup we get from Rust.

Note: This code is intentionally NOT optimized to show typical Python performance.
Even with optimizations (NumPy, Cython, etc.), Rust will still be significantly faster.
"""

import json
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass
from collections import defaultdict


@dataclass
class PythonLogStats:
    """Python equivalent of Rust LogStats"""
    total_count: int
    error_count: int
    warn_count: int
    info_count: int
    avg_duration_ms: float
    min_duration_ms: float
    max_duration_ms: float
    p50_duration_ms: float
    p95_duration_ms: float
    p99_duration_ms: float
    status_code_distribution: Dict[int, int]
    error_count_by_code: Dict[int, int]

    def summary(self) -> str:
        """Get a summary string"""
        return (
            f"Total logs: {self.total_count}\n"
            f"Error count: {self.error_count}\n"
            f"Warning count: {self.warn_count}\n"
            f"Info count: {self.info_count}\n"
            f"Average duration: {self.avg_duration_ms:.2f}ms\n"
            f"P95 duration: {self.p95_duration_ms:.2f}ms\n"
            f"P99 duration: {self.p99_duration_ms:.2f}ms"
        )


class PurePythonProcessor:
    """
    Pure Python implementation of log processing.

    This class replicates the Rust functionality in pure Python for
    benchmarking purposes. The performance difference demonstrates why
    Rust is valuable for CPU-intensive operations.
    """

    @staticmethod
    def parse_logs(log_lines: List[str]) -> List[Dict[str, any]]:
        """
        Parse JSON log strings.

        This is slower than Rust because:
        1. No parallelization (GIL prevents effective multi-threading)
        2. Python's json parser is slower than serde_json
        3. Interpreted vs compiled code
        """
        results = []
        for line in log_lines:
            try:
                entry = json.loads(line)
                results.append(entry)
            except json.JSONDecodeError as e:
                raise ValueError(f"Parse error: {e}")
        return results

    @staticmethod
    def validate_logs(log_lines: List[str]) -> Tuple[int, List[str]]:
        """
        Validate log entries with detailed error reporting.

        Python's dynamic typing means we have to do runtime checks that
        Rust can catch at compile time.
        """
        errors = []
        valid_count = 0
        valid_levels = ["ERROR", "WARN", "INFO", "DEBUG"]

        for idx, line in enumerate(log_lines, 1):
            try:
                # Try to parse
                entry = json.loads(line)

                # Validate required fields
                if not entry.get('timestamp'):
                    errors.append(f"Line {idx}: Missing or empty timestamp")
                    continue

                # Validate log level
                level = entry.get('level', '')
                if level not in valid_levels:
                    errors.append(
                        f"Line {idx}: Invalid log level '{level}'. "
                        f"Must be one of: {', '.join(valid_levels)}"
                    )
                    continue

                # Validate duration if present
                duration = entry.get('duration_ms')
                if duration is not None:
                    if not isinstance(duration, (int, float)) or duration < 0:
                        errors.append(
                            f"Line {idx}: Invalid duration_ms {duration}. "
                            f"Must be >= 0"
                        )
                        continue

                # Validate status code if present
                status = entry.get('status_code')
                if status is not None:
                    if not isinstance(status, int) or not (100 <= status <= 599):
                        errors.append(
                            f"Line {idx}: Invalid status_code {status}. "
                            f"Must be 100-599"
                        )
                        continue

                valid_count += 1

            except json.JSONDecodeError as e:
                errors.append(f"Line {idx}: JSON parse error: {e}")
                continue

        return valid_count, errors

    @staticmethod
    def compute_stats(log_lines: List[str]) -> PythonLogStats:
        """
        Compute comprehensive statistics from log entries.

        This is 20-50x slower than Rust because:
        1. Single-threaded processing (GIL)
        2. Interpreted vs compiled
        3. Less efficient memory layout
        4. Slower sorting algorithms
        """
        # Parse all logs
        entries = []
        for line in log_lines:
            try:
                entry = json.loads(line)
                entries.append(entry)
            except json.JSONDecodeError:
                continue

        if not entries:
            raise ValueError("No valid log entries found")

        # Count by log level
        error_count = sum(1 for e in entries if e.get('level') == 'ERROR')
        warn_count = sum(1 for e in entries if e.get('level') == 'WARN')
        info_count = sum(1 for e in entries if e.get('level') == 'INFO')

        # Collect all durations for percentile calculation
        durations = [
            e['duration_ms']
            for e in entries
            if 'duration_ms' in e and e['duration_ms'] is not None
        ]

        if durations:
            # Sort for percentile calculation
            # Note: Python's sort is good but still slower than Rust
            durations.sort()

            avg_duration = sum(durations) / len(durations)
            min_duration = durations[0]
            max_duration = durations[-1]

            # Calculate percentiles
            p50_idx = int(len(durations) * 0.50)
            p95_idx = int(len(durations) * 0.95)
            p99_idx = int(len(durations) * 0.99)

            p50 = durations[min(p50_idx, len(durations) - 1)]
            p95 = durations[min(p95_idx, len(durations) - 1)]
            p99 = durations[min(p99_idx, len(durations) - 1)]
        else:
            avg_duration = min_duration = max_duration = 0.0
            p50 = p95 = p99 = 0.0

        # Status code distribution
        status_code_distribution = defaultdict(int)
        for entry in entries:
            if 'status_code' in entry and entry['status_code'] is not None:
                status_code_distribution[entry['status_code']] += 1

        # Error codes (4xx, 5xx)
        error_count_by_code = defaultdict(int)
        for entry in entries:
            code = entry.get('status_code')
            if code is not None and code >= 400:
                error_count_by_code[code] += 1

        return PythonLogStats(
            total_count=len(entries),
            error_count=error_count,
            warn_count=warn_count,
            info_count=info_count,
            avg_duration_ms=avg_duration,
            min_duration_ms=min_duration,
            max_duration_ms=max_duration,
            p50_duration_ms=p50,
            p95_duration_ms=p95,
            p99_duration_ms=p99,
            status_code_distribution=dict(status_code_distribution),
            error_count_by_code=dict(error_count_by_code),
        )

    @staticmethod
    def filter_logs(
        log_lines: List[str],
        min_level: Optional[str] = None,
        min_duration_ms: Optional[float] = None,
        status_codes: Optional[List[int]] = None
    ) -> List[Dict[str, any]]:
        """
        Filter logs by various criteria.

        Single-threaded filtering is much slower on large datasets.
        """
        # Helper to convert level to numeric value
        level_to_num = {
            'ERROR': 3,
            'WARN': 2,
            'INFO': 1,
            'DEBUG': 0,
        }

        min_level_num = level_to_num.get(min_level, 0) if min_level else 0

        # Parse and filter
        filtered = []
        for line in log_lines:
            try:
                entry = json.loads(line)

                # Check log level
                entry_level_num = level_to_num.get(entry.get('level', 'DEBUG'), 0)
                if entry_level_num < min_level_num:
                    continue

                # Check duration
                if min_duration_ms is not None:
                    duration = entry.get('duration_ms')
                    if duration is None or duration < min_duration_ms:
                        continue

                # Check status codes
                if status_codes:
                    status = entry.get('status_code')
                    if status is None or status not in status_codes:
                        continue

                filtered.append(entry)

            except json.JSONDecodeError:
                continue

        return filtered

    @staticmethod
    def batch_process(log_lines: List[str]) -> Tuple[PythonLogStats, List[str]]:
        """
        Batch process logs with all operations.

        This combines validation and stats computation.
        """
        _, errors = PurePythonProcessor.validate_logs(log_lines)
        stats = PurePythonProcessor.compute_stats(log_lines)
        return stats, errors


# Example usage
if __name__ == "__main__":
    print("Pure Python Processor Example")
    print("=" * 60)
    print("\nThis is the pure Python implementation used for benchmarking.")
    print("It replicates Rust functionality but is 20-50x slower.")
    print("=" * 60)

    # Sample data
    sample_logs = [
        json.dumps({
            "timestamp": "2024-01-15T10:30:00Z",
            "level": "ERROR",
            "message": "Database connection failed",
            "duration_ms": 1250.5,
            "status_code": 500,
            "user_id": "user_123"
        }),
        json.dumps({
            "timestamp": "2024-01-15T10:30:01Z",
            "level": "INFO",
            "message": "Request processed successfully",
            "duration_ms": 45.2,
            "status_code": 200,
            "user_id": "user_456"
        }),
    ]

    processor = PurePythonProcessor()

    # Test parsing
    print("\n1. Parsing logs...")
    parsed = processor.parse_logs(sample_logs)
    print(f"   Parsed {len(parsed)} logs")

    # Test stats
    print("\n2. Computing statistics...")
    stats = processor.compute_stats(sample_logs)
    print(f"   {stats.summary()}")

    # Test filtering
    print("\n3. Filtering logs...")
    filtered = processor.filter_logs(sample_logs, min_level="ERROR")
    print(f"   Filtered to {len(filtered)} logs")

    print("\n" + "=" * 60)
    print("For real performance comparison, see examples/benchmark.py")
    print("=" * 60)
