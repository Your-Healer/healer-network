#!/bin/bash

# VPS Setup Script for Healer Network Node
# Run this script on your Ubuntu VPS to configure firewall and networking

echo "=== VPS Setup for Healer Network Node ==="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if running as root or with sudo
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}This script must be run as root or with sudo${NC}"
   echo "Usage: sudo ./vps-setup.sh"
   exit 1
fi

# Update system packages
echo -e "${BLUE}1. Updating system packages...${NC}"
apt update && apt upgrade -y

# Install required packages
echo -e "${BLUE}2. Installing required packages...${NC}"
apt install -y curl wget git build-essential jq netcat-openbsd

# Install Docker if not present
if ! command -v docker > /dev/null 2>&1; then
    echo -e "${BLUE}3. Installing Docker...${NC}"
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
    usermod -aG docker $SUDO_USER
    systemctl enable docker
    systemctl start docker
    rm get-docker.sh
    echo -e "${GREEN}✓ Docker installed${NC}"
else
    echo -e "${GREEN}✓ Docker already installed${NC}"
fi

# Configure UFW firewall
echo -e "${BLUE}4. Configuring firewall (UFW)...${NC}"
if command -v ufw > /dev/null 2>&1; then
    # Allow SSH (make sure we don't lock ourselves out)
    ufw allow ssh
    ufw allow 22
    
    # Allow Healer Network ports
    ufw allow 9933/tcp comment "Healer Network RPC HTTP"
    ufw allow 9944/tcp comment "Healer Network WebSocket"
    ufw allow 9615/tcp comment "Healer Network Prometheus"
    ufw allow 30333/tcp comment "Healer Network P2P"
    
    # Enable UFW if not already enabled
    ufw --force enable
    
    echo -e "${GREEN}✓ Firewall configured${NC}"
    echo -e "${YELLOW}Allowed ports:${NC}"
    ufw status numbered
else
    echo -e "${RED}UFW not available, please configure your firewall manually${NC}"
fi

# Configure Docker daemon for better performance
echo -e "${BLUE}5. Optimizing Docker configuration...${NC}"
mkdir -p /etc/docker
cat > /etc/docker/daemon.json << EOF
{
    "log-driver": "json-file",
    "log-opts": {
        "max-size": "10m",
        "max-file": "3"
    },
    "storage-driver": "overlay2"
}
EOF
systemctl restart docker

# Check system resources
echo -e "${BLUE}6. System resource check...${NC}"
echo -e "${YELLOW}CPU cores:${NC} $(nproc)"
echo -e "${YELLOW}Memory:${NC} $(free -h | awk '/^Mem:/ {print $2}')"
echo -e "${YELLOW}Disk space:${NC} $(df -h / | awk 'NR==2 {print $4}')"

# Optimize system for blockchain node
echo -e "${BLUE}7. Applying system optimizations...${NC}"
# Increase file descriptor limits
echo "* soft nofile 65536" >> /etc/security/limits.conf
echo "* hard nofile 65536" >> /etc/security/limits.conf
echo "root soft nofile 65536" >> /etc/security/limits.conf
echo "root hard nofile 65536" >> /etc/security/limits.conf

# Optimize kernel parameters
cat >> /etc/sysctl.conf << EOF
# Optimizations for blockchain node
net.core.rmem_max = 16777216
net.core.wmem_max = 16777216
net.ipv4.tcp_rmem = 4096 87380 16777216
net.ipv4.tcp_wmem = 4096 65536 16777216
fs.file-max = 2097152
EOF
sysctl -p

echo ""
echo -e "${GREEN}=== VPS Setup Complete! ===${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "${BLUE}1.${NC} Make scripts executable: chmod +x *.sh"
echo -e "${BLUE}2.${NC} Build Docker image: ./build-docker.sh"
echo -e "${BLUE}3.${NC} Run the node: ./run-docker.sh"
echo -e "${BLUE}4.${NC} Test connectivity: ./test-connection.sh"
echo -e "${BLUE}5.${NC} Check external access from your local machine:"
echo "   curl -X POST -H 'Content-Type: application/json' \\"
echo "   -d '{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}' \\"
echo "   http://YOUR_VPS_IP:9933"
echo ""
echo -e "${YELLOW}Important:${NC}"
echo -e "${RED}• Replace YOUR_VPS_IP with your actual VPS public IP address${NC}"
echo -e "${RED}• Make sure your cloud provider allows inbound traffic on ports 9933, 9944, 9615, 30333${NC}"
echo -e "${RED}• You may need to log out and back in for Docker group changes to take effect${NC}"
