#!/bin/bash

# Run this script from the root of the repository.

set -e

# Create a timestamped log folder
LOG_DIR="$(pwd)/logs/$(date +'%Y%m%d-%H%M%S')"
mkdir -p "$LOG_DIR"

# cargo build --release --bin gnarkctl
# cp target/release/gnarkctl gnarkctl

export CHUNK_SIZE=4194304
export CHUNK_BATCH_SIZE=12
export SPLIT_THRESHOLD=1048576
export RUST_LOG=info
export RUSTFLAGS="-C target-cpu=native -C target-feature=+avx512f,+avx512ifma,+avx512vl"
# JEMALLOC does not work properly on aws
# export JEMALLOC_SYS_WITH_MALLOC_CONF="retain:true,background_thread:true,metadata_thp:always,dirty_decay_ms:-1,muzzy_decay_ms:-1,abort_conf:true"
export VK_VERIFICATION=false

# PROGRAMS=("aster-10user" "aster-100user" "aster-1000user" "aster-3000user")
# FIELD="kb"

RUNS=2

# ./gnarkctl setup --field $FIELD

cd ./prover

for i in $(seq 1 $RUNS); do
  # for PROG in "${PROGRAMS[@]}"; do
  echo "===== Run #$i ====="
  LOG_FILE="bench_run${i}.log"
  # cargo run --profile perf --features jemalloc,nightly-features | tee "$LOG_DIR/$LOG_FILE"
  cargo run --profile perf | tee "$LOG_DIR/$LOG_FILE"
  # done
done

cd ..
# ./gnarkctl teardown
# rm gnarkctl

echo "pico benchmark aster (kb) completed!"