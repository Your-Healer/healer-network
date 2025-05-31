# PowerShell script to troubleshoot healer-network Docker connectivity

Write-Host "=== Healer Network Docker Troubleshooting ===" -ForegroundColor Yellow
Write-Host ""

# Check if Docker is running
Write-Host "1. Checking Docker status..." -ForegroundColor Cyan
try {
    docker version | Out-Null
    Write-Host "✓ Docker is running" -ForegroundColor Green
} catch {
    Write-Host "✗ Docker is not running or not accessible" -ForegroundColor Red
    exit 1
}

# Check if the image exists
Write-Host ""
Write-Host "2. Checking if healer-network image exists..." -ForegroundColor Cyan
$imageExists = docker images --format "table {{.Repository}}:{{.Tag}}" | Select-String "healer-network:latest"
if ($imageExists) {
    Write-Host "✓ healer-network:latest image found" -ForegroundColor Green
} else {
    Write-Host "✗ healer-network:latest image not found" -ForegroundColor Red
    Write-Host "Run '.\build-docker.ps1' to build the image first" -ForegroundColor Yellow
}

# Check if container is running
Write-Host ""
Write-Host "3. Checking container status..." -ForegroundColor Cyan
$containerStatus = docker ps -a --format "table {{.Names}}\t{{.Status}}" | Select-String "healer-network-node"
if ($containerStatus) {
    Write-Host "Container status: $containerStatus" -ForegroundColor Yellow
} else {
    Write-Host "✗ No healer-network-node container found" -ForegroundColor Red
}

# Check port availability
Write-Host ""
Write-Host "4. Checking port availability..." -ForegroundColor Cyan
$ports = @(9944, 9933, 9615, 30333)
foreach ($port in $ports) {
    try {
        $connection = Test-NetConnection -ComputerName 0.0.0.0 -Port $port -InformationLevel Quiet -WarningAction SilentlyContinue
        if ($connection) {
            Write-Host "✓ Port $port is accessible" -ForegroundColor Green
        } else {
            Write-Host "✗ Port $port is not accessible" -ForegroundColor Red
        }
    } catch {
        Write-Host "✗ Cannot test port $port" -ForegroundColor Red
    }
}

# Show container logs if running
Write-Host ""
Write-Host "5. Container logs (last 20 lines)..." -ForegroundColor Cyan
try {
    docker logs --tail 20 healer-network-node 2>$null
} catch {
    Write-Host "No logs available or container not found" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== Troubleshooting Commands ===" -ForegroundColor Yellow
Write-Host "View live logs: docker logs -f healer-network-node" -ForegroundColor Cyan
Write-Host "Stop container: docker stop healer-network-node" -ForegroundColor Cyan
Write-Host "Remove container: docker rm healer-network-node" -ForegroundColor Cyan
Write-Host "Rebuild image: .\build-docker.ps1" -ForegroundColor Cyan
Write-Host "Start container: .\run-docker.ps1" -ForegroundColor Cyan
Write-Host ""
Write-Host "Test RPC connection: curl http://0.0.0.0:9933 -X POST -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}'" -ForegroundColor Cyan
