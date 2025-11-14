"""
Log Processing Pipeline - Production-Ready Example

This module demonstrates how to structure a real-world application that uses
Rust for performance-critical operations while keeping business logic in Python.

Pattern: Python for I/O and orchestration, Rust for CPU-intensive processing
"""

import json
import logging
from pathlib import Path
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
from datetime import datetime

# Import the Rust module (will be available after building with maturin)
try:
    import rust_processor
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
    print("Warning: rust_processor not available. Build with 'make build' first.")


@dataclass
class ProcessingResult:
    """Results from log processing pipeline"""
    stats: Optional[object]  # LogStats object from Rust
    errors: List[str]
    total_processed: int
    processing_time_ms: float
    filtered_count: int = 0


class LogPipeline:
    """
    Production-ready log processing pipeline.

    This class demonstrates best practices for integrating Rust into a Python
    application:
    - Python handles I/O (file/network operations)
    - Python handles business logic (routing, alerting, etc.)
    - Rust handles CPU-intensive work (parsing, stats, filtering)
    - Proper error handling and logging throughout
    """

    def __init__(self, error_threshold: int = 100):
        """
        Initialize the pipeline.

        Args:
            error_threshold: Number of errors that triggers an alert
        """
        self.error_threshold = error_threshold
        self.logger = logging.getLogger(__name__)

        if not RUST_AVAILABLE:
            raise RuntimeError(
                "Rust processor not available. "
                "Build it first with: cd rust_processor && maturin develop"
            )

    def load_logs_from_file(self, file_path: Path) -> List[str]:
        """
        Load log entries from a file.

        This is Python's strength - I/O operations. We handle file reading,
        error handling, and preprocessing here.

        Args:
            file_path: Path to log file (JSONL format)

        Returns:
            List of JSON log strings
        """
        self.logger.info(f"Loading logs from {file_path}")

        if not file_path.exists():
            raise FileNotFoundError(f"Log file not found: {file_path}")

        logs = []
        with open(file_path, 'r') as f:
            for line in f:
                line = line.strip()
                if line:  # Skip empty lines
                    logs.append(line)

        self.logger.info(f"Loaded {len(logs)} log entries")
        return logs

    def process_batch(self, log_lines: List[str]) -> ProcessingResult:
        """
        Process a batch of logs using Rust for heavy lifting.

        This method demonstrates the key integration pattern:
        1. Python prepares the data
        2. Rust does CPU-intensive processing
        3. Python handles the results

        Args:
            log_lines: List of JSON log strings

        Returns:
            ProcessingResult with stats and errors
        """
        import time

        self.logger.info(f"Processing {len(log_lines)} logs...")
        start_time = time.time()

        try:
            # Call Rust for batch processing - this is where the speedup happens!
            # Rust will parse, validate, and compute stats in parallel
            stats, errors = rust_processor.batch_process(log_lines)

            processing_time = (time.time() - start_time) * 1000  # Convert to ms

            self.logger.info(
                f"Processed {stats.total_count} logs in {processing_time:.2f}ms "
                f"({len(errors)} errors)"
            )

            return ProcessingResult(
                stats=stats,
                errors=errors,
                total_processed=stats.total_count,
                processing_time_ms=processing_time
            )

        except Exception as e:
            self.logger.error(f"Processing failed: {e}")
            raise

    def filter_high_severity_logs(
        self,
        log_lines: List[str],
        min_level: str = "WARN",
        min_duration_ms: Optional[float] = None
    ) -> List[Dict[str, str]]:
        """
        Filter logs by severity and duration using Rust.

        This demonstrates how to use Rust for complex filtering that would
        be slow in pure Python, especially with millions of records.

        Args:
            log_lines: List of JSON log strings
            min_level: Minimum log level (ERROR, WARN, INFO, DEBUG)
            min_duration_ms: Minimum duration threshold

        Returns:
            Filtered log entries
        """
        self.logger.info(
            f"Filtering logs (level>={min_level}, duration>={min_duration_ms}ms)"
        )

        filtered = rust_processor.filter_logs(
            log_lines,
            min_level=min_level,
            min_duration_ms=min_duration_ms,
            status_codes=None
        )

        self.logger.info(f"Filtered to {len(filtered)} logs")
        return filtered

    def filter_error_status_codes(
        self,
        log_lines: List[str],
        status_codes: List[int]
    ) -> List[Dict[str, str]]:
        """
        Filter logs by HTTP status codes.

        Args:
            log_lines: List of JSON log strings
            status_codes: List of status codes to include (e.g., [500, 502, 503])

        Returns:
            Filtered log entries
        """
        self.logger.info(f"Filtering logs by status codes: {status_codes}")

        filtered = rust_processor.filter_logs(
            log_lines,
            min_level=None,
            min_duration_ms=None,
            status_codes=status_codes
        )

        self.logger.info(f"Found {len(filtered)} logs with specified status codes")
        return filtered

    def analyze_and_alert(self, file_path: Path) -> ProcessingResult:
        """
        Complete pipeline: load, process, and potentially alert.

        This demonstrates a real-world use case where:
        - Python handles I/O (loading from file/S3/database)
        - Rust handles processing (parsing, stats, filtering)
        - Python handles business logic (alerting, routing)

        Args:
            file_path: Path to log file

        Returns:
            ProcessingResult
        """
        # Step 1: Python handles I/O
        log_lines = self.load_logs_from_file(file_path)

        # Step 2: Rust handles CPU-intensive processing
        result = self.process_batch(log_lines)

        # Step 3: Python handles business logic
        if len(result.errors) > self.error_threshold:
            self._send_alert(result)

        if result.stats and result.stats.error_count > 0:
            # Filter error logs for detailed analysis
            error_logs = self.filter_high_severity_logs(
                log_lines,
                min_level="ERROR"
            )
            result.filtered_count = len(error_logs)
            self._analyze_errors(error_logs)

        return result

    def _send_alert(self, result: ProcessingResult):
        """
        Send alert when error threshold is exceeded.

        This is Python's domain - integrating with external services,
        sending notifications, etc.
        """
        self.logger.warning(
            f"ALERT: Error threshold exceeded! "
            f"Errors: {len(result.errors)}, Threshold: {self.error_threshold}"
        )

        # In production, you would:
        # - Send email/Slack notification
        # - Create incident ticket
        # - Update monitoring dashboard
        # - Trigger automated remediation

        print(f"\n{'='*60}")
        print(f"⚠️  ALERT: High Error Rate Detected!")
        print(f"{'='*60}")
        print(f"Total errors: {len(result.errors)}")
        print(f"Threshold: {self.error_threshold}")
        print(f"First 5 errors:")
        for error in result.errors[:5]:
            print(f"  - {error}")
        print(f"{'='*60}\n")

    def _analyze_errors(self, error_logs: List[Dict[str, str]]):
        """
        Analyze error logs for patterns.

        This demonstrates Python's strength in complex business logic
        while Rust handled the heavy filtering.
        """
        self.logger.info(f"Analyzing {len(error_logs)} error logs...")

        # Python is great for this kind of flexible analysis
        error_messages = {}
        for log in error_logs:
            msg = log.get('message', 'Unknown')
            error_messages[msg] = error_messages.get(msg, 0) + 1

        # Find most common errors
        sorted_errors = sorted(
            error_messages.items(),
            key=lambda x: x[1],
            reverse=True
        )

        print(f"\nTop 5 Error Messages:")
        for msg, count in sorted_errors[:5]:
            print(f"  [{count:4}x] {msg[:80]}")

    def get_stats_summary(self, log_lines: List[str]) -> str:
        """
        Get a formatted summary of log statistics.

        Args:
            log_lines: List of JSON log strings

        Returns:
            Formatted summary string
        """
        stats = rust_processor.compute_stats(log_lines)
        return stats.summary()


# Example usage demonstrating the pipeline
if __name__ == "__main__":
    # Configure logging
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )

    print("Log Processing Pipeline Example")
    print("=" * 60)
    print("\nThis example demonstrates Python + Rust integration:")
    print("- Python: I/O, business logic, orchestration")
    print("- Rust: CPU-intensive parsing, stats, filtering")
    print("=" * 60)

    # Create sample data for demonstration
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
        json.dumps({
            "timestamp": "2024-01-15T10:30:02Z",
            "level": "WARN",
            "message": "High memory usage detected",
            "duration_ms": 890.0,
            "status_code": 200,
            "user_id": "user_789"
        }),
    ]

    try:
        pipeline = LogPipeline(error_threshold=5)

        # Process logs
        result = pipeline.process_batch(sample_logs)

        print(f"\nProcessing Results:")
        print(f"  Total processed: {result.total_processed}")
        print(f"  Processing time: {result.processing_time_ms:.2f}ms")
        print(f"  Validation errors: {len(result.errors)}")

        if result.stats:
            print(f"\n{result.stats.summary()}")

    except RuntimeError as e:
        print(f"\n⚠️  {e}")
        print("\nTo build the Rust module, run:")
        print("  cd rust_processor")
        print("  maturin develop")
