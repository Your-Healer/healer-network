# Remove any existing container with the same name
docker stop healer-network-node 2>/dev/null
docker rm healer-network-node 2>/dev/null

# Run the healer-network node with proper network configuration
docker run -d \
  --name healer-network-node \
  -p 9944:9944 \
  -p 9933:9933 \
  -p 9615:9615 \
  -p 30333:30333 \
  -v healer-network-data:/data \
  healer-network:latest \
  --rpc-cors=all \
  --unsafe-rpc-external \
  --rpc-methods=unsafe \
  --rpc-external \
  --prometheus-external