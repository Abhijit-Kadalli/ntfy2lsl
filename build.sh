#!/bin/bash
set -e

# Download dependencies first
cargo fetch

# Find and patch the problematic file
find /usr/local/cargo/registry/src -name "thread_data.hpp" -exec \
  sed -i 's/#if PTHREAD_STACK_MIN > 0/#if defined(PTHREAD_STACK_MIN) \&\& PTHREAD_STACK_MIN > 0/g' {} \;

# Build the project
cargo build --release 