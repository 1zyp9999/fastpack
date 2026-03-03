#!/bin/bash
# Example C++ project for FastPack demonstration

echo "Building example C++ project..."

mkdir -p build
cd build

cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)

echo "Build complete!"
echo "Run ./build/example_app to test the application"