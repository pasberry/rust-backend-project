#!/usr/bin/env python3
"""
Basic Usage Examples for Rust + Python Integration

This script demonstrates 5 simple examples showing how to use the Rust
processor from Python. These examples progress from simple to more complex.
"""

import json
import sys
from pathlib import Path

# Add parent directory to path to import our modules
sys.path.insert(0, str(Path(__file__).parent.parent))

try:
    import rust_processor
except ImportError:
    print("❌ Error: rust_processor module not found!")
    print("\nPlease build the Rust module first:")
    print("  cd rust_processor")
    print("  maturin develop")
    print("\nOr use the Makefile:")
    print("  make build")
    sys.exit(1)


def print_example_header(number: int, title: str):
    """Print a formatted example header"""
    print(f"\n{'='*70}")
    print(f"Example {number}: {title}")
    print(f"{'='*70}\n")


def example1_parse_logs():
    """Example 1: Basic log parsing"""
    print_example_header(1, "Basic Log Parsing")

    # Create sample log entries
    log_lines = [
        json.dumps({
            "timestamp": "2024-01-15T10:30:00Z",
            "level": "INFO",
            "message": "User logged in",
            "duration_ms": 45.2,
            "status_code": 200,
            "user_id": "user_123"
        }),
        json.dumps({
            "timestamp": "2024-01-15T10:30:01Z",
            "level": "ERROR",
            "message": "Database connection failed",
            "duration_ms": 1250.5,
            "status_code": 500,
            "user_id": "user_456"
        }),
        json.dumps({
            "timestamp": "2024-01-15T10:30:02Z",
            "level": "WARN",
            "message": "High memory usage",
            "duration_ms": 890.0,
            "status_code": 200,
            "user_id": "user_789"
        }),
    ]

    # Parse logs using Rust (parallel processing!)
    parsed_logs = rust_processor.parse_logs(log_lines)

    print(f"✅ Successfully parsed {len(parsed_logs)} log entries\n")

    for i, log in enumerate(parsed_logs, 1):
        print(f"Log {i}:")
        print(f"  Level: {log['level']}")
        print(f"  Message: {log['message']}")
        print(f"  Duration: {log.get('duration_ms', 'N/A')}ms")
        print()


def example2_validate_logs():
    """Example 2: Log validation with error detection"""
    print_example_header(2, "Log Validation with Error Detection")

    # Mix of valid and invalid logs
    log_lines = [
        json.dumps({
            "timestamp": "2024-01-15T10:30:00Z",
            "level": "INFO",
            "message": "Valid log",
            "duration_ms": 45.2,
            "status_code": 200,
        }),
        json.dumps({
            "timestamp": "",  # Invalid: empty timestamp
            "level": "ERROR",
            "message": "Invalid log 1",
        }),
        json.dumps({
            "timestamp": "2024-01-15T10:30:01Z",
            "level": "INVALID_LEVEL",  # Invalid: bad log level
            "message": "Invalid log 2",
        }),
        json.dumps({
            "timestamp": "2024-01-15T10:30:02Z",
            "level": "INFO",
            "message": "Another valid log",
            "duration_ms": -10.0,  # Invalid: negative duration
        }),
    ]

    # Validate using Rust
    valid_count, errors = rust_processor.validate_logs(log_lines)

    print(f"✅ Validation complete!")
    print(f"   Valid logs: {valid_count}")
    print(f"   Invalid logs: {len(errors)}\n")

    if errors:
        print("Validation errors:")
        for error in errors:
            print(f"  ❌ {error}")


def example3_compute_stats():
    """Example 3: Computing statistics"""
    print_example_header(3, "Computing Statistics (The Speed Demo!)")

    # Generate more log entries to show performance
    log_lines = []
    for i in range(1000):
        log_lines.append(json.dumps({
            "timestamp": f"2024-01-15T10:30:{i%60:02d}Z",
            "level": ["INFO", "WARN", "ERROR"][i % 3],
            "message": f"Log message {i}",
            "duration_ms": 10.0 + (i % 100) * 5.0,
            "status_code": [200, 404, 500][i % 3],
            "user_id": f"user_{i % 10}"
        }))

    print(f"Processing {len(log_lines)} log entries...\n")

    # Compute stats using Rust (parallel processing!)
    stats = rust_processor.compute_stats(log_lines)

    print(f"✅ Statistics computed!\n")
    print(stats.summary())
    print(f"\nStatus code distribution:")
    for code, count in sorted(stats.status_code_distribution.items()):
        print(f"  {code}: {count} occurrences")


def example4_filter_logs():
    """Example 4: Filtering logs by criteria"""
    print_example_header(4, "Filtering Logs by Criteria")

    # Create diverse log entries
    log_lines = []
    levels = ["DEBUG", "INFO", "WARN", "ERROR"]
    status_codes = [200, 201, 400, 404, 500, 502]

    for i in range(100):
        log_lines.append(json.dumps({
            "timestamp": f"2024-01-15T10:30:{i%60:02d}Z",
            "level": levels[i % len(levels)],
            "message": f"Log message {i}",
            "duration_ms": 10.0 + (i % 50) * 10.0,
            "status_code": status_codes[i % len(status_codes)],
            "user_id": f"user_{i % 5}"
        }))

    print(f"Starting with {len(log_lines)} log entries\n")

    # Filter 1: Only errors
    print("Filter 1: Only ERROR level logs")
    errors_only = rust_processor.filter_logs(
        log_lines,
        min_level="ERROR",
        min_duration_ms=None,
        status_codes=None
    )
    print(f"  Result: {len(errors_only)} logs\n")

    # Filter 2: Slow requests (> 200ms)
    print("Filter 2: Slow requests (duration > 200ms)")
    slow_requests = rust_processor.filter_logs(
        log_lines,
        min_level=None,
        min_duration_ms=200.0,
        status_codes=None
    )
    print(f"  Result: {len(slow_requests)} logs\n")

    # Filter 3: Server errors (5xx status codes)
    print("Filter 3: Server errors (status 500, 502)")
    server_errors = rust_processor.filter_logs(
        log_lines,
        min_level=None,
        min_duration_ms=None,
        status_codes=[500, 502]
    )
    print(f"  Result: {len(server_errors)} logs\n")

    # Filter 4: Combined - slow errors
    print("Filter 4: Slow ERROR level logs (duration > 100ms)")
    slow_errors = rust_processor.filter_logs(
        log_lines,
        min_level="ERROR",
        min_duration_ms=100.0,
        status_codes=None
    )
    print(f"  Result: {len(slow_errors)} logs")


def example5_batch_process():
    """Example 5: Batch processing (validation + stats in one call)"""
    print_example_header(5, "Batch Processing (Most Efficient!)")

    # Create a mix of valid and invalid logs
    log_lines = []

    # Add valid logs
    for i in range(50):
        log_lines.append(json.dumps({
            "timestamp": f"2024-01-15T10:30:{i%60:02d}Z",
            "level": ["INFO", "WARN", "ERROR"][i % 3],
            "message": f"Log message {i}",
            "duration_ms": 10.0 + (i % 30) * 5.0,
            "status_code": [200, 404, 500][i % 3],
        }))

    # Add a few invalid logs
    log_lines.append(json.dumps({
        "timestamp": "",
        "level": "ERROR",
        "message": "Invalid: empty timestamp",
    }))
    log_lines.append(json.dumps({
        "timestamp": "2024-01-15T10:30:00Z",
        "level": "BADLEVEL",
        "message": "Invalid: bad level",
    }))

    print(f"Processing {len(log_lines)} log entries in a single batch call...\n")

    # Batch process: combines validation and stats computation
    # This is more efficient than calling validate_logs() and compute_stats() separately
    stats, errors = rust_processor.batch_process(log_lines)

    print(f"✅ Batch processing complete!\n")
    print(f"Statistics:")
    print(f"  {stats}")
    print(f"\nValidation:")
    print(f"  Total entries: {len(log_lines)}")
    print(f"  Valid entries: {stats.total_count}")
    print(f"  Validation errors: {len(errors)}")

    if errors:
        print(f"\nFirst 3 validation errors:")
        for error in errors[:3]:
            print(f"  ❌ {error}")

    print("\n💡 Tip: batch_process() is more efficient than calling")
    print("   validate_logs() and compute_stats() separately because")
    print("   it reduces the overhead of crossing the Python-Rust boundary.")


def main():
    """Run all examples"""
    print("\n" + "="*70)
    print("  Rust + Python Integration: Basic Usage Examples")
    print("="*70)
    print("\nThese examples demonstrate how to use Rust for CPU-intensive")
    print("operations while keeping your application logic in Python.")
    print("\nKey benefits:")
    print("  • 20-50x faster processing")
    print("  • Automatic parallelization with Rayon")
    print("  • Type safety from Rust")
    print("  • Easy integration with PyO3")

    try:
        example1_parse_logs()
        example2_validate_logs()
        example3_compute_stats()
        example4_filter_logs()
        example5_batch_process()

        print("\n" + "="*70)
        print("  ✅ All examples completed successfully!")
        print("="*70)
        print("\nNext steps:")
        print("  • Run 'python examples/benchmark.py' to see performance comparison")
        print("  • Check 'python_orchestrator/log_processor/pipeline.py' for")
        print("    production-ready integration patterns")
        print("  • Read the documentation in README.md and docs/")
        print("="*70 + "\n")

    except Exception as e:
        print(f"\n❌ Error running examples: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
