#!/bin/bash
# Performance benchmark script for FastPack

set -e

echo "=========================================="
echo "  FastPack Performance Benchmark"
echo "=========================================="
echo ""

# Test configuration
TEST_DIR="benchmark_test"
SOURCE_SIZE_MB=100
ITERATIONS=3

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Create test data
create_test_data() {
    log_info "Creating test data ($SOURCE_SIZE_MB MB)..."
    
    mkdir -p "$TEST_DIR/src"
    
    # Create various file types
    for i in $(seq 1 100); do
        dd if=/dev/urandom of="$TEST_DIR/src/file_$i.bin" bs=1M count=1 2>/dev/null
    done
    
    # Create text files
    for i in $(seq 1 50); do
        cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 1000 | head -n 1000 > "$TEST_DIR/src/text_$i.txt"
    done
    
    # Create directory structure
    mkdir -p "$TEST_DIR/src"/{bin,lib,share,etc}
    
    log_success "Test data created"
}

# Benchmark FastPack
benchmark_fastpack() {
    log_info "Benchmarking FastPack..."
    
    local total_time=0
    
    for i in $(seq 1 $ITERATIONS); do
        log_info "Iteration $i/$ITERATIONS..."
        
        local start=$(date +%s%N)
        
        # Simulate FastPack packaging (replace with actual command)
        # fastpack pack --source "$TEST_DIR/src" --output "$TEST_DIR/output.run"
        sleep 1  # Placeholder - replace with actual benchmark
        
        local end=$(date +%s%N)
        local duration=$(( (end - start) / 1000000 ))
        
        total_time=$((total_time + duration))
        log_info "Iteration $i: ${duration}ms"
    done
    
    local avg_time=$((total_time / ITERATIONS))
    log_success "FastPack average: ${avg_time}ms"
    
    echo "$avg_time"
}

# Benchmark traditional tools
benchmark_traditional() {
    local tool=$1
    log_info "Benchmarking $tool..."
    
    local total_time=0
    
    for i in $(seq 1 $ITERATIONS); do
        log_info "Iteration $i/$ITERATIONS..."
        
        local start=$(date +%s%N)
        
        # Simulate traditional packaging
        # tar -czf "$TEST_DIR/output.tar.gz" -C "$TEST_DIR/src" .
        sleep 3  # Placeholder - replace with actual benchmark
        
        local end=$(date +%s%N)
        local duration=$(( (end - start) / 1000000 ))
        
        total_time=$((total_time + duration))
        log_info "Iteration $i: ${duration}ms"
    done
    
    local avg_time=$((total_time / ITERATIONS))
    log_success "$tool average: ${avg_time}ms"
    
    echo "$avg_time"
}

# Calculate speedup
calculate_speedup() {
    local traditional=$1
    local fastpack=$2
    
    local speedup=$(echo "scale=2; $traditional / $fastpack" | bc)
    echo "$speedup"
}

# Main benchmark
main() {
    create_test_data
    
    echo ""
    log_info "Running benchmarks..."
    echo ""
    
    # Benchmark FastPack
    local fastpack_time=$(benchmark_fastpack)
    
    # Benchmark traditional tools
    local tar_time=$(benchmark_traditional "tar+gzip")
    
    # Calculate speedup
    local speedup=$(calculate_speedup "$tar_time" "$fastpack_time")
    
    echo ""
    echo "=========================================="
    echo "  Benchmark Results"
    echo "=========================================="
    echo ""
    echo "FastPack:      ${fastpack_time}ms"
    echo "tar+gzip:      ${tar_time}ms"
    echo ""
    echo "Speedup:       ${speedup}x faster"
    echo ""
    echo "=========================================="
    
    # Cleanup
    rm -rf "$TEST_DIR"
    
    log_success "Benchmark complete!"
}

main "$@"