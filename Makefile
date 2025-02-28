# Makefile for Vestra

# Variables
BINARY_NAME = vestra
BUILD_DIR = target/release
SOURCE_FILES = $(wildcard src/**/*.rs src/*.rs)

# Default target
all: build

# Build the project
build:
	cargo build --release

# Run the Vestra script
run: build
	$(BUILD_DIR)/$(BINARY_NAME) run examples/example.vs

# Clean the project
clean:
	cargo clean
	find . -name '*.d' -delete

# Rebuild the project
rebuild: clean build

# Watch for changes and rebuild
watch:
	cargo watch -x 'build --release'

# Phony targets
.PHONY: all build run clean rebuild watch
