#!/bin/bash

# Simple connection test for healer-network node

echo "Testing Healer Network Node Connectivity..."
echo "==========================================="

# Test RPC HTTP endpoint
echo "Testing RPC HTTP endpoint (port 9933)..."
response=$(curl -s -w "%{http_code}" -X POST -H "Content-Type: application/json" \
   -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
   http://0.0.0.0:9933)

http_code="${response: -3}"
if [ "$http_code" = "200" ]; then
    echo "✓ RPC HTTP endpoint is working!"
    echo "Response: ${response%???}"
else
    echo "✗ RPC HTTP endpoint failed (HTTP code: $http_code)"
fi

echo ""

# Test WebSocket endpoint (if wscat is available)
if command -v wscat > /dev/null 2>&1; then
    echo "Testing WebSocket endpoint (port 9944)..."
    timeout 5 wscat -c ws://0.0.0.0:9944 --execute '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' 2>/dev/null && echo "✓ WebSocket endpoint is working!" || echo "✗ WebSocket endpoint failed"
else
    echo "WebSocket test skipped (wscat not installed)"
fi

echo ""

# Test Prometheus metrics
echo "Testing Prometheus metrics (port 9615)..."
metrics_response=$(curl -s -w "%{http_code}" http://0.0.0.0:9615/metrics)
metrics_code="${metrics_response: -3}"
if [ "$metrics_code" = "200" ]; then
    echo "✓ Prometheus metrics endpoint is working!"
else
    echo "✗ Prometheus metrics endpoint failed (HTTP code: $metrics_code)"
fi

echo ""

# Show node info if RPC is working
if [ "$http_code" = "200" ]; then
    echo "Getting node information..."
    curl -s -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_name","params":[],"id":1}' \
        http://0.0.0.0:9933 | jq '.' 2>/dev/null || echo "jq not installed, raw response above"
    
    echo ""
    echo "Getting chain info..."
    curl -s -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_chain","params":[],"id":1}' \
        http://0.0.0.0:9933 | jq '.' 2>/dev/null || echo "jq not installed, raw response above"
fi

echo ""
echo "Connection test completed!"
