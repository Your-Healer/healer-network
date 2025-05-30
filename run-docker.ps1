# PowerShell script to run healer-network Docker container

# Remove any existing container with the same name
Write-Host "Stopping and removing existing container (if any)..."
docker stop healer-network-node 2>$null
docker rm healer-network-node 2>$null

Write-Host "Starting healer-network container..."

# Run the healer-network node with proper network configuration
docker run -d `
  --name healer-network-node `
  -p 9944:9944 `
  -p 9933:9933 `
  -p 9615:9615 `
  -p 30333:30333 `
  -v healer-network-data:/data `
  healer-network:latest `
  --base-path=/data `
  --rpc-cors=all `
  --unsafe-rpc-external `
  --rpc-methods=unsafe `
  --rpc-external `
  --ws-external `
  --listen-addr=/ip4/0.0.0.0/tcp/30333 `
  --dev

if ($LASTEXITCODE -eq 0) {
    Write-Host "Container started successfully!"
    Write-Host "RPC endpoint: ws://localhost:9944"
    Write-Host "HTTP RPC endpoint: http://localhost:9933"
    Write-Host "Prometheus metrics: http://localhost:9615"
    Write-Host ""
    Write-Host "To view logs: docker logs -f healer-network-node"
    Write-Host "To stop: docker stop healer-network-node"
} else {
    Write-Host "Failed to start container. Check Docker logs for details."
}
