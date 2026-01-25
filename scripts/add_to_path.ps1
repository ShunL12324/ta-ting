# 将开发工具添加到 Windows PATH 环境变量
# 需要以管理员身份运行 PowerShell

Write-Host "=== 添加开发工具到 PATH ===" -ForegroundColor Green
Write-Host ""

# 需要添加的路径
$paths = @(
    "C:\Users\Shun\.cargo\bin",           # Rust/Cargo
    "C:\Program Files\CMake\bin",         # CMake
    "C:\Program Files\LLVM\bin"           # LLVM
)

# 获取当前用户的 PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

$added = 0
$skipped = 0

foreach ($path in $paths) {
    # 检查路径是否存在
    if (-not (Test-Path $path)) {
        Write-Host "[SKIP] 路径不存在: $path" -ForegroundColor Yellow
        $skipped++
        continue
    }

    # 检查是否已在 PATH 中
    if ($currentPath -like "*$path*") {
        Write-Host "[OK] 已存在: $path" -ForegroundColor Cyan
        $skipped++
    } else {
        Write-Host "[ADD] 添加: $path" -ForegroundColor Green
        $currentPath = $currentPath + ";" + $path
        $added++
    }
}

# 如果有新添加的路径，更新环境变量
if ($added -gt 0) {
    Write-Host ""
    Write-Host "正在更新用户环境变量..." -ForegroundColor Yellow
    
    [Environment]::SetEnvironmentVariable("Path", $currentPath, "User")
    
    Write-Host "✅ 成功添加 $added 个路径到 PATH" -ForegroundColor Green
    Write-Host ""
    Write-Host "⚠️  重要提示:" -ForegroundColor Yellow
    Write-Host "1. 请关闭并重新打开 PowerShell 窗口" -ForegroundColor White
    Write-Host "2. 或者重启电脑使更改生效" -ForegroundColor White
} else {
    Write-Host ""
    Write-Host "✅ 所有路径已在 PATH 中，无需更新" -ForegroundColor Green
}

Write-Host ""
Write-Host "=== 完成 ===" -ForegroundColor Green
