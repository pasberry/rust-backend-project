"""
Log Processor - Python Orchestration Layer

This package demonstrates Pattern 2: Python handles I/O, business logic,
and orchestration while Rust handles CPU-intensive processing.
"""

from .pipeline import LogPipeline
from .pure_python import PurePythonProcessor

__all__ = ['LogPipeline', 'PurePythonProcessor']
__version__ = '0.1.0'
