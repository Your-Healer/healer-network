#!/bin/bash

echo "Building healer-network Docker image..."
docker build -t healer-network:latest .

echo "Build completed. Image tagged as 'healer-network:latest'"
docker images | grep healer-network
