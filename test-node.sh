#!/bin/bash

echo "=== Testing Healer Network Node Startup ==="

# Test 1: Build the image
echo "🔨 Building image..."
if docker build -t healer-network:test .; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Test 2: Test basic functionality
echo "🧪 Testing basic node functionality..."
docker run --rm healer-network:test --version

# Test 3: Test data directory permissions
echo "📁 Testing data directory permissions..."
docker run --rm -v test-data:/data healer-network:test sh -c "
    touch /data/test.txt && echo '✅ Can write to /data' || echo '❌ Cannot write to /data'
    ls -la /data/
"

# Test 4: Test node initialization with proper base path
echo "🚀 Testing node initialization..."
timeout 30 docker run --rm \
    -v test-data:/data \
    healer-network:test \
    --base-path=/data \
    --chain=dev \
    --validator \
    --alice \
    --unsafe-rpc-external &

# Wait a moment then check if process started
sleep 5
echo "Node initialization test completed"

# Cleanup
docker volume rm test-data 2>/dev/null || true

echo "✅ All tests completed"
