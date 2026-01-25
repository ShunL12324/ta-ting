# TaTing 启动脚本
# 自动清理端口并启动应用

Write-Host "🚀 正在启动 TaTing..." -ForegroundColor Cyan

# 1. 停止占用 5173 端口的进程
Write-Host "🔍 检查端口 5173..." -ForegroundColor Yellow
$port = 5173
$connections = netstat -ano | Select-String ":$port\s" | Select-String "LISTENING"

if ($connections) {
    Write-Host "⚠️  发现端口占用，正在清理..." -ForegroundColor Yellow
    $connections | ForEach-Object {
        $line = $_.Line
        if ($line -match "\s+(\d+)$") {
            $pid = $matches[1]
            if ($pid -ne 0) {
                Write-Host "   停止进程 PID: $pid" -ForegroundColor Gray
                Stop-Process -Id $pid -Force -ErrorAction SilentlyContinue
            }
        }
    }
    Start-Sleep -Seconds 1
}

# 2. 停止所有 node 进程（以防万一）
Write-Host "🧹 清理旧的 node 进程..." -ForegroundColor Yellow
Stop-Process -Name node -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

# 3. 确认端口已释放
$check = netstat -ano | Select-String ":$port\s" | Select-String "LISTENING"
if ($check) {
    Write-Host "❌ 端口仍然被占用，请手动检查" -ForegroundColor Red
    exit 1
}

Write-Host "✅ 端口已释放" -ForegroundColor Green

# 4. 启动 Tauri 开发服务器
Write-Host ""
Write-Host "🎬 启动 TaTing 开发服务器..." -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""

npm run tauri:dev
