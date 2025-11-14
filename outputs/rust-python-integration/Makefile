.PHONY: help install build build-release test benchmark clean format lint example data

# Default target
help:
	@echo "Rust + Python Integration - Available Commands"
	@echo "==============================================="
	@echo ""
	@echo "Setup & Build:"
	@echo "  make install        Install dependencies (maturin, etc.)"
	@echo "  make build          Build Rust module in debug mode"
	@echo "  make build-release  Build Rust module in release mode (optimized)"
	@echo ""
	@echo "Testing & Benchmarking:"
	@echo "  make test           Run Python tests"
	@echo "  make example        Run basic usage examples"
	@echo "  make benchmark      Run performance benchmarks"
	@echo "  make benchmark-full Run comprehensive benchmarks"
	@echo ""
	@echo "Data Generation:"
	@echo "  make data           Generate sample data files"
	@echo ""
	@echo "Code Quality:"
	@echo "  make format         Format Python code with black"
	@echo "  make lint           Run linting checks"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean          Remove build artifacts"
	@echo ""
	@echo "Quick Start:"
	@echo "  make install && make build && make example"
	@echo ""

# Install dependencies
install:
	@echo "Installing dependencies..."
	pip install -r requirements.txt
	@echo "✅ Dependencies installed"

# Build in debug mode (faster compilation, slower runtime)
build:
	@echo "Building Rust module in debug mode..."
	cd rust_processor && maturin develop
	@echo "✅ Build complete (debug mode)"

# Build in release mode (slower compilation, faster runtime)
build-release:
	@echo "Building Rust module in release mode..."
	cd rust_processor && maturin develop --release
	@echo "✅ Build complete (release mode - optimized)"

# Run tests
test:
	@echo "Running tests..."
	pytest tests/ -v
	@echo "✅ Tests complete"

# Run basic examples
example:
	@echo "Running basic usage examples..."
	python examples/basic_usage.py
	@echo "✅ Examples complete"

# Run quick benchmark
benchmark:
	@echo "Running quick benchmark..."
	python examples/benchmark.py
	@echo "✅ Benchmark complete"

# Run comprehensive benchmark
benchmark-full:
	@echo "Running comprehensive benchmark..."
	python examples/benchmark.py --full
	@echo "✅ Comprehensive benchmark complete"

# Run large dataset benchmark (includes 1M records)
benchmark-large:
	@echo "Running large dataset benchmark (this will take a while)..."
	python examples/benchmark.py --large
	@echo "✅ Large benchmark complete"

# Generate sample data
data:
	@echo "Generating sample data files..."
	python examples/generate_sample_data.py --preset
	@echo "✅ Sample data generated"

# Format Python code
format:
	@echo "Formatting Python code..."
	black python_orchestrator/ examples/ --line-length 100
	@echo "✅ Formatting complete"

# Run linting
lint:
	@echo "Running linting checks..."
	black python_orchestrator/ examples/ --check --line-length 100
	mypy python_orchestrator/
	@echo "✅ Linting complete"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf rust_processor/target
	rm -rf build
	rm -rf dist
	rm -rf *.egg-info
	rm -rf .pytest_cache
	rm -rf __pycache__
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete
	find . -type f -name "*.so" -delete
	rm -rf data/*.jsonl
	@echo "✅ Cleanup complete"

# Quick start - install, build, and run example
quickstart: install build example
	@echo ""
	@echo "✅ Quick start complete!"
	@echo ""
	@echo "Next steps:"
	@echo "  make benchmark      - See performance comparison"
	@echo "  make data           - Generate sample data"
	@echo ""

# Full setup - install, build release, generate data, run benchmark
full-setup: install build-release data benchmark-full
	@echo ""
	@echo "✅ Full setup complete!"
	@echo ""
	@echo "Everything is ready to use. Check the documentation in README.md"
	@echo ""
