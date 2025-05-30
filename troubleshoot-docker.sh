#!/bin/bash

# Troubleshooting script for healer-network Docker connectivity on Ubuntu VPS

echo "=== Healer Network Docker Troubleshooting ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if Docker is running
echo -e "${BLUE}1. Checking Docker status...${NC}"
if docker version > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Docker is running${NC}"
else
    echo -e "${RED}✗ Docker is not running or not accessible${NC}"
    exit 1
fi

# Check if the image exists
echo ""
echo -e "${BLUE}2. Checking if healer-network image exists...${NC}"
if docker images | grep -q "healer-network.*latest"; then
    echo -e "${GREEN}✓ healer-network:latest image found${NC}"
else
    echo -e "${RED}✗ healer-network:latest image not found${NC}"
    echo -e "${YELLOW}Run './build-docker.sh' to build the image first${NC}"
fi

# Check if container is running
echo ""
echo -e "${BLUE}3. Checking container status...${NC}"
if docker ps -a | grep -q "healer-network-node"; then
    container_status=$(docker ps -a --format "table {{.Names}}\t{{.Status}}" | grep healer-network-node)
    echo -e "${YELLOW}Container status: $container_status${NC}"
else
    echo -e "${RED}✗ No healer-network-node container found${NC}"
fi

# Check port availability
echo ""
echo -e "${BLUE}4. Checking port accessibility...${NC}"
ports=(9944 9933 9615 30333)
for port in "${ports[@]}"; do
    if nc -z localhost $port 2>/dev/null; then
        echo -e "${GREEN}✓ Port $port is accessible${NC}"
    else
        echo -e "${RED}✗ Port $port is not accessible${NC}"
    fi
done

# Check firewall status (if UFW is installed)
echo ""
echo -e "${BLUE}5. Checking firewall status...${NC}"
if command -v ufw > /dev/null 2>&1; then
    ufw_status=$(sudo ufw status 2>/dev/null | head -n 1)
    echo -e "${YELLOW}UFW Status: $ufw_status${NC}"
    
    if [[ "$ufw_status" == *"active"* ]]; then
        echo -e "${YELLOW}UFW is active. Make sure ports 9944, 9933, 9615, 30333 are allowed:${NC}"
        echo "sudo ufw allow 9944"
        echo "sudo ufw allow 9933"
        echo "sudo ufw allow 9615"
        echo "sudo ufw allow 30333"
    fi
else
    echo -e "${YELLOW}UFW not installed${NC}"
fi

# Test RPC connection
echo ""
echo -e "${BLUE}6. Testing RPC connection...${NC}"
if curl -s -X POST -H "Content-Type: application/json" \
   -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
   http://localhost:9933 > /dev/null 2>&1; then
    echo -e "${GREEN}✓ RPC endpoint is responding${NC}"
else
    echo -e "${RED}✗ RPC endpoint is not responding${NC}"
fi

# Test WebSocket connection
echo ""
echo -e "${BLUE}7. Testing WebSocket connection...${NC}"
if command -v wscat > /dev/null 2>&1; then
    # Use timeout to prevent hanging
    if timeout 5 wscat -c ws://localhost:9944 --execute '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' > /dev/null 2>&1; then
        echo -e "${GREEN}✓ WebSocket endpoint is responding${NC}"
    else
        echo -e "${RED}✗ WebSocket endpoint is not responding${NC}"
    fi
else
    echo -e "${YELLOW}wscat not installed. Install with: npm install -g wscat${NC}"
fi

# Show container logs if running
echo ""
echo -e "${BLUE}8. Container logs (last 20 lines)...${NC}"
if docker ps | grep -q "healer-network-node"; then
    docker logs --tail 20 healer-network-node 2>/dev/null
else
    echo -e "${YELLOW}Container is not running${NC}"
fi

echo ""
echo -e "${YELLOW}=== Troubleshooting Commands ===${NC}"
echo -e "${BLUE}View live logs:${NC} docker logs -f healer-network-node"
echo -e "${BLUE}Stop container:${NC} docker stop healer-network-node"
echo -e "${BLUE}Remove container:${NC} docker rm healer-network-node"
echo -e "${BLUE}Rebuild image:${NC} ./build-docker.sh"
echo -e "${BLUE}Start container:${NC} ./run-docker.sh"
echo ""
echo -e "${BLUE}Test RPC:${NC} curl -X POST -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}' http://localhost:9933"
echo -e "${BLUE}Test from external:${NC} curl -X POST -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}' http://YOUR_VPS_IP:9933"
echo ""
echo -e "${YELLOW}=== For VPS External Access ===${NC}"
echo -e "${BLUE}1. Make sure your VPS firewall allows the ports${NC}"
echo -e "${BLUE}2. Check your cloud provider's security groups/firewall rules${NC}"
echo -e "${BLUE}3. Replace YOUR_VPS_IP with your actual VPS IP address${NC}"
