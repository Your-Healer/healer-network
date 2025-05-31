#!/bin/bash

# Make all shell scripts executable

echo "Making shell scripts executable..."

chmod +x build-docker.sh
chmod +x run-docker.sh
chmod +x troubleshoot-docker.sh
chmod +x test-connection.sh
chmod +x vps-setup.sh
chmod +x start-node.sh

echo "✓ All shell scripts are now executable"
echo ""
echo "Available scripts:"
echo "  ./vps-setup.sh       - Initial VPS setup (run with sudo)"
echo "  ./build-docker.sh    - Build Docker image"
echo "  ./run-docker.sh      - Run Docker container"
echo "  ./test-connection.sh - Test node connectivity"
echo "  ./troubleshoot-docker.sh - Comprehensive troubleshooting"
echo "  ./start-node.sh      - Start node locally (non-Docker)"
