# PowerShell script to clean up Docker containers and volumes

Write-Host "=== Cleaning up Docker environment ===" -ForegroundColor Yellow

# Stop and remove container
Write-Host "Stopping and removing healer-network-node container..." -ForegroundColor Cyan
docker stop healer-network-node 2>$null
docker rm healer-network-node 2>$null

# Remove the problematic volume (this will delete all data!)
Write-Host "Removing Docker volume (this will delete all blockchain data)..." -ForegroundColor Red
$confirm = Read-Host "Are you sure you want to delete the volume 'healer-network-data'? (y/N)"
if ($confirm -eq "y" -or $confirm -eq "Y") {
    docker volume rm healer-network-data 2>$null
    Write-Host "Volume removed successfully" -ForegroundColor Green
} else {
    Write-Host "Volume cleanup skipped" -ForegroundColor Yellow
}

# Clean up unused images and containers
Write-Host "Cleaning up unused Docker resources..." -ForegroundColor Cyan
docker system prune -f

Write-Host "Cleanup completed!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Rebuild the image: .\build-docker.ps1" -ForegroundColor Cyan
Write-Host "2. Run the container: .\run-docker.ps1" -ForegroundColor Cyan
