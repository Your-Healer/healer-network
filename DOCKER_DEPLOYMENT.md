# Healer Network Docker Deployment Guide

This guide helps you deploy the Healer Network node using Docker on a VPS Ubuntu server.

## Quick Start

### 1. VPS Setup (First time only)

```bash
# Make scripts executable
chmod +x *.sh

# Run VPS setup (requires sudo)
sudo ./vps-setup.sh
```

### 2. Build and Run

```bash
# Build the Docker image
./build-docker.sh

# Run the container
./run-docker.sh

# Test connectivity
./test-connection.sh
```

## Troubleshooting Connection Issues

### Common Problems and Solutions

#### 1. Node starts but not accessible externally

**Problem**: Node runs locally but can't connect from outside the VPS.

**Solution**:

```bash
# Check if ports are bound correctly
./troubleshoot-docker.sh

# Verify firewall settings
sudo ufw status

# Test local connection first
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
  http://0.0.0.0:9933
```

#### 2. Docker container exits immediately

**Problem**: Container starts then stops.

**Solution**:

```bash
# Check container logs
docker logs healer-network-node

# Common issues:
# - Binary not found: Check if build was successful
# - Permission issues: Check user permissions in Dockerfile
# - Port conflicts: Check if ports are already in use
```

#### 3. RPC/WebSocket not responding

**Problem**: Node runs but RPC endpoints don't respond.

**Solution**:

```bash
# Check if node is binding to all interfaces
docker logs healer-network-node | grep -i "rpc\|websocket"

# Verify Docker port mapping
docker port healer-network-node

# Check if services are listening
netstat -tlnp | grep -E ":(9933|9944|9615|30333)"
```

## Configuration Options

### Development Mode (Default)

```bash
docker run -d \
  --name healer-network-node \
  -p 9944:9944 -p 9933:9933 -p 9615:9615 -p 30333:30333 \
  healer-network:latest \
  --dev --validator --alice
```

### Production Mode

```bash
docker run -d \
  --name healer-network-node \
  -p 9944:9944 -p 9933:9933 -p 9615:9615 -p 30333:30333 \
  -v healer-network-data:/data \
  healer-network:latest \
  --base-path=/data \
  --chain=local \
  --validator \
  --rpc-external --ws-external \
  --rpc-bind-address=0.0.0.0 \
  --ws-bind-address=0.0.0.0
```

## Port Reference

| Port  | Service     | External Access |
| ----- | ----------- | --------------- |
| 9933  | RPC HTTP    | ✓ (if enabled)  |
| 9944  | WebSocket   | ✓ (if enabled)  |
| 9615  | Prometheus  | ✓ (if enabled)  |
| 30333 | P2P Network | ✓ (always)      |

## Security Considerations

### For Development

- Uses `--dev` flag with pre-funded accounts
- Enables unsafe RPC methods
- No authentication required

### For Production

- Remove `--dev` flag
- Remove `--unsafe-rpc-external` and `--rpc-methods=unsafe`
- Set up proper authentication
- Use HTTPS/WSS with reverse proxy
- Restrict RPC access to trusted IPs

## Advanced Configuration

### Custom Chain Specification

```bash
# Generate custom chain spec
docker run --rm -v $(pwd):/output healer-network:latest \
  build-spec --chain=dev --raw > /output/chain-spec.json

# Use custom chain spec
docker run -d \
  --name healer-network-node \
  -v $(pwd)/chain-spec.json:/chain-spec.json \
  -v healer-network-data:/data \
  healer-network:latest \
  --base-path=/data \
  --chain=/chain-spec.json
```

### Monitoring with Prometheus

```bash
# Start with Prometheus metrics
docker run -d \
  --name healer-network-node \
  -p 9615:9615 \
  healer-network:latest \
  --prometheus-external \
  --prometheus-bind-address=0.0.0.0

# Access metrics
curl http://your-vps-ip:9615/metrics
```

## Docker Compose (Optional)

Create `docker-compose.yml`:

```yaml
version: "3.8"
services:
  healer-network:
    image: healer-network:latest
    container_name: healer-network-node
    ports:
      - "9944:9944"
      - "9933:9933"
      - "9615:9615"
      - "30333:30333"
    volumes:
      - healer-network-data:/data
    command:
      [
        "--base-path=/data",
        "--chain=dev",
        "--validator",
        "--alice",
        "--rpc-external",
        "--ws-external",
        "--rpc-bind-address=0.0.0.0",
        "--ws-bind-address=0.0.0.0",
        "--prometheus-external",
        "--prometheus-bind-address=0.0.0.0",
      ]
    restart: unless-stopped

volumes:
  healer-network-data:
```

Run with:

```bash
docker-compose up -d
```

## Testing External Connectivity

### From Local Machine

```bash
# Replace YOUR_VPS_IP with actual IP
export VPS_IP="your.vps.ip.address"

# Test RPC
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
  http://$VPS_IP:9933

# Test WebSocket (requires wscat: npm install -g wscat)
wscat -c ws://$VPS_IP:9944 \
  --execute '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}'
```

### Using Polkadot.js Apps

1. Open [Polkadot.js Apps](https://polkadot.js.org/apps/)
2. Click on the network selector (top left)
3. Choose "Custom" and enter: `ws://your-vps-ip:9944`
4. Click "Switch"

## Useful Commands

```bash
# View logs
docker logs -f healer-network-node

# Enter container
docker exec -it healer-network-node /bin/sh

# Stop and remove
docker stop healer-network-node
docker rm healer-network-node

# Rebuild image
docker build -t healer-network:latest .

# Check resource usage
docker stats healer-network-node

# Backup data
docker run --rm -v healer-network-data:/source -v $(pwd):/backup \
  ubuntu tar czf /backup/healer-network-backup.tar.gz /source
```

## Performance Tuning

### System Resources

- **Minimum**: 2 CPU cores, 4GB RAM, 50GB storage
- **Recommended**: 4+ CPU cores, 8GB+ RAM, 100GB+ SSD

### Docker Optimizations

```bash
# Increase container memory (if needed)
docker run --memory=4g --cpus=2 ...

# Use host networking for better performance (less secure)
docker run --network=host ...
```
