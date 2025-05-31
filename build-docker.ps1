# PowerShell script to build healer-network Docker image

Write-Host "Building healer-network Docker image..." -ForegroundColor Green

docker build -t healer-network:latest .

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build completed successfully! Image tagged as 'healer-network:latest'" -ForegroundColor Green
    Write-Host ""
    Write-Host "Available healer-network images:" -ForegroundColor Yellow
    docker images | Select-String "healer-network"
    Write-Host ""
    Write-Host "To run the container: .\run-docker.ps1" -ForegroundColor Cyan
} else {
    Write-Host "Build failed. Please check the error messages above." -ForegroundColor Red
}
