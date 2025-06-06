#!/bin/bash

echo "Cleaning up old Docker resources..."
# Remove old healer-network images
docker rmi healer-network:latest 2>/dev/null || true
docker image prune -f

# Clean up Docker build cache
docker builder prune -f

# Remove any dangling images and containers
docker system prune -f

echo "Building healer-network Docker image..."
docker build -t healer-network:latest .

echo "Build completed. Image tagged as 'healer-network:latest'"
docker images | grep healer-network
