#!/bin/bash

echo "=== Docker Debug Script for Healer Network ==="
echo "This script will help debug the permission and connectivity issues"
echo

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check Docker installation
if ! command_exists docker; then
    echo "❌ Docker is not installed or not in PATH"
    exit 1
fi

echo "✅ Docker is installed"
docker --version
echo

# Clean up previous containers and volumes
echo "🧹 Cleaning up previous containers and volumes..."
docker stop healer-network-node 2>/dev/null || true
docker rm healer-network-node 2>/dev/null || true
docker volume rm healer-network-data 2>/dev/null || true
echo

# Build the image
echo "🔨 Building Docker image..."
if docker build -t healer-network:latest .; then
    echo "✅ Docker image built successfully"
else
    echo "❌ Failed to build Docker image"
    exit 1
fi
echo

# Create a fresh volume with proper permissions
echo "📁 Creating fresh data volume..."
docker volume create healer-network-data
echo

# Test permissions by running a simple command
echo "🔍 Testing file permissions in container..."
docker run --rm \
    -v healer-network-data:/data \
    healer-network:latest \
    sh -c "
        echo 'Current user:' && id
        echo 'Data directory permissions:' && ls -la /data
        echo 'Creating test file...'
        if touch /data/test-file; then
            echo '✅ Write permissions work'
            rm /data/test-file
        else
            echo '❌ Write permissions failed'
            exit 1
        fi
        echo 'Testing node version:'
        /usr/local/bin/healer-network-node --version
    "

if [ $? -eq 0 ]; then
    echo "✅ Permission test passed"
else
    echo "❌ Permission test failed"
    exit 1
fi
echo

# Run the node with debug output
echo "🚀 Starting node with debug output..."
docker run -d \
    --name healer-network-node \
    -p 9944:9944 \
    -p 9933:9933 \
    -p 9615:9615 \
    -p 30333:30333 \
    -v healer-network-data:/data \
    healer-network:latest \
    --base-path=/data \
    --chain=dev \
    --rpc-cors=all \
    --unsafe-rpc-external \
    --rpc-methods=unsafe \
    --rpc-external \
    --validator \
    --alice \
    -l info

echo "✅ Container started. Container ID:"
docker ps | grep healer-network-node
echo

# Wait a few seconds for initialization
echo "⏳ Waiting 10 seconds for node initialization..."
sleep 10

# Check logs
echo "📋 Recent logs:"
docker logs healer-network-node --tail 50
echo

# Check if ports are listening
echo "🔌 Checking if ports are accessible..."
echo "Testing RPC port 9933:"
if curl -s -X POST -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' http://localhost:9933 >/dev/null; then
    echo "✅ RPC port 9933 is accessible"
else
    echo "❌ RPC port 9933 is not accessible"
fi

echo "Testing WebSocket port 9944:"
if curl -s -I http://localhost:9944 >/dev/null; then
    echo "✅ WebSocket port 9944 is accessible"
else
    echo "❌ WebSocket port 9944 is not accessible"
fi

echo
echo "🎯 Debug complete. If there are still issues, check the logs above."
echo "To continue monitoring logs: docker logs -f healer-network-node"
echo "To stop the container: docker stop healer-network-node"
