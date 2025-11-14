#!/usr/bin/env python3
"""
Generate Sample Log Data

This script generates realistic log data files for testing and benchmarking.
It creates JSONL (JSON Lines) format files with configurable sizes.
"""

import json
import random
import argparse
from pathlib import Path
from datetime import datetime, timedelta
from typing import List


# Realistic log messages by level
ERROR_MESSAGES = [
    "Database connection timeout",
    "Failed to process payment",
    "Authentication service unavailable",
    "Out of memory error",
    "Null pointer exception in handler",
    "Failed to write to disk",
    "API rate limit exceeded",
    "Invalid JWT token",
    "Session expired",
    "Connection refused to Redis",
]

WARN_MESSAGES = [
    "High memory usage detected",
    "Slow database query",
    "Cache miss rate above threshold",
    "Retry attempt 3 of 5",
    "Deprecated API endpoint used",
    "Large file upload detected",
    "Connection pool nearly exhausted",
    "SSL certificate expires soon",
    "Background job queue growing",
    "Unusual traffic pattern detected",
]

INFO_MESSAGES = [
    "User logged in successfully",
    "Request completed",
    "Database query executed",
    "Cache hit for key",
    "File uploaded successfully",
    "Email sent to user",
    "Background job completed",
    "Session created",
    "API request processed",
    "Configuration reloaded",
]

DEBUG_MESSAGES = [
    "Entering function processRequest",
    "Cache lookup for key",
    "Validating user input",
    "Parsing request headers",
    "Loading configuration",
    "Initializing connection pool",
    "Checking permissions",
    "Serializing response",
    "Updating metrics",
    "Logging audit trail",
]


def generate_log_entry(index: int, start_time: datetime) -> dict:
    """Generate a single realistic log entry"""

    # Determine log level (realistic distribution)
    level_choice = random.random()
    if level_choice < 0.05:  # 5% errors
        level = "ERROR"
        messages = ERROR_MESSAGES
        status_codes = [500, 502, 503, 504]
        duration_range = (500, 5000)  # Errors tend to be slower
    elif level_choice < 0.20:  # 15% warnings
        level = "WARN"
        messages = WARN_MESSAGES
        status_codes = [200, 400, 401, 403, 404, 429]
        duration_range = (100, 2000)
    elif level_choice < 0.70:  # 50% info
        level = "INFO"
        messages = INFO_MESSAGES
        status_codes = [200, 201, 202, 204]
        duration_range = (10, 500)
    else:  # 30% debug
        level = "DEBUG"
        messages = DEBUG_MESSAGES
        status_codes = [200]
        duration_range = (1, 100)

    # Generate timestamp (spread over time)
    timestamp = start_time + timedelta(seconds=index * 0.1)  # 10 logs per second

    # Generate realistic duration (with occasional outliers)
    if random.random() < 0.95:
        duration = random.uniform(*duration_range)
    else:
        # 5% outliers
        duration = random.uniform(duration_range[1], duration_range[1] * 5)

    # Create log entry
    entry = {
        "timestamp": timestamp.isoformat() + "Z",
        "level": level,
        "message": random.choice(messages),
        "duration_ms": round(duration, 2),
        "status_code": random.choice(status_codes),
        "user_id": f"user_{random.randint(1, 10000)}",
    }

    # Add optional fields randomly
    if random.random() < 0.3:  # 30% have request_id
        entry["request_id"] = f"req_{random.randint(100000, 999999)}"

    if random.random() < 0.4:  # 40% have endpoint
        endpoints = ["/api/users", "/api/orders", "/api/products", "/api/auth", "/api/search"]
        entry["endpoint"] = random.choice(endpoints)

    if random.random() < 0.2:  # 20% have error_code
        if level in ["ERROR", "WARN"]:
            entry["error_code"] = f"ERR_{random.randint(1000, 9999)}"

    return entry


def generate_log_file(
    output_path: Path,
    count: int,
    start_time: datetime = None
) -> None:
    """
    Generate a log file with specified number of entries.

    Args:
        output_path: Path where the file will be created
        count: Number of log entries to generate
        start_time: Starting timestamp (defaults to now)
    """
    if start_time is None:
        start_time = datetime.now()

    print(f"Generating {count:,} log entries...")
    print(f"Output file: {output_path}")

    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, 'w') as f:
        for i in range(count):
            entry = generate_log_entry(i, start_time)
            f.write(json.dumps(entry) + '\n')

            # Progress indicator
            if (i + 1) % 10000 == 0:
                print(f"  Generated {i + 1:,} / {count:,} entries...")

    file_size_mb = output_path.stat().st_size / (1024 * 1024)
    print(f"\n✅ Successfully generated {count:,} log entries")
    print(f"   File size: {file_size_mb:.2f} MB")
    print(f"   Location: {output_path}")


def generate_preset_files():
    """Generate preset sample files with different sizes"""
    presets = [
        ("sample_1k.jsonl", 1_000, "Small test file"),
        ("sample_10k.jsonl", 10_000, "Medium test file"),
        ("sample_100k.jsonl", 100_000, "Large test file"),
        ("sample_1m.jsonl", 1_000_000, "Extra large test file"),
    ]

    data_dir = Path(__file__).parent.parent / "data"
    start_time = datetime(2024, 1, 15, 10, 0, 0)

    print("\n" + "="*70)
    print("  Generating Preset Sample Files")
    print("="*70 + "\n")

    for filename, count, description in presets:
        print(f"\n{description}:")
        print("-" * 70)
        output_path = data_dir / filename
        generate_log_file(output_path, count, start_time)

    print("\n" + "="*70)
    print("  ✅ All preset files generated successfully!")
    print("="*70)
    print(f"\nFiles created in: {data_dir}")
    print("\nUsage examples:")
    print("  python examples/basic_usage.py")
    print("  python examples/benchmark.py --full")
    print("="*70 + "\n")


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="Generate sample log data for testing and benchmarking",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Generate preset files (1K, 10K, 100K, 1M)
  python generate_sample_data.py --preset

  # Generate custom file with 50K entries
  python generate_sample_data.py --count 50000 --output data/custom.jsonl

  # Generate 1M entries for stress testing
  python generate_sample_data.py --count 1000000 --output data/stress_test.jsonl
        """
    )

    parser.add_argument(
        "--preset",
        action="store_true",
        help="Generate preset sample files (1K, 10K, 100K, 1M)"
    )

    parser.add_argument(
        "--count",
        type=int,
        help="Number of log entries to generate"
    )

    parser.add_argument(
        "--output",
        type=Path,
        help="Output file path (JSONL format)"
    )

    parser.add_argument(
        "--start-date",
        type=str,
        help="Start date/time (ISO format, default: now)"
    )

    args = parser.parse_args()

    if args.preset:
        generate_preset_files()
    elif args.count and args.output:
        start_time = None
        if args.start_date:
            try:
                start_time = datetime.fromisoformat(args.start_date.replace('Z', ''))
            except ValueError:
                print(f"❌ Invalid date format: {args.start_date}")
                print("   Use ISO format: 2024-01-15T10:00:00")
                return 1

        generate_log_file(args.output, args.count, start_time)
    else:
        parser.print_help()
        print("\nQuick start:")
        print("  python generate_sample_data.py --preset")

    return 0


if __name__ == "__main__":
    exit(main())
