# TaTing 模型下载脚本 (PowerShell)

$ErrorActionPreference = "Stop"

Write-Host "=== TaTing 模型下载脚本 ===" -ForegroundColor Cyan
Write-Host ""

$MODEL_NAME = "sherpa-onnx-zipformer-multi-zh-hans-2023-9-2"
$MODEL_URL = "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/${MODEL_NAME}.tar.bz2"
$TARGET_DIR = "src-tauri\resources\models\sherpa-zh"
$DOWNLOAD_FILE = "${MODEL_NAME}.tar.bz2"

# 检查目标目录
if (!(Test-Path $TARGET_DIR)) {
    Write-Host "✗ 错误: 目标目录不存在: $TARGET_DIR" -ForegroundColor Red
    Write-Host "  请确保在项目根目录运行此脚本" -ForegroundColor Yellow
    exit 1
}

# 检查模型是否已存在
if (Test-Path "$TARGET_DIR\$MODEL_NAME") {
    Write-Host "✓ 模型已存在: $TARGET_DIR\$MODEL_NAME" -ForegroundColor Green
    $response = Read-Host "是否重新下载? (y/N)"
    if ($response -ne 'y' -and $response -ne 'Y') {
        Write-Host "跳过下载" -ForegroundColor Yellow
        exit 0
    }
    Remove-Item -Path "$TARGET_DIR\$MODEL_NAME" -Recurse -Force
}

# 下载模型
Write-Host "📥 开始下载模型..." -ForegroundColor Cyan
Write-Host "   URL: $MODEL_URL"
Write-Host "   大小: 约 100MB (压缩后)"
Write-Host ""

try {
    # 使用 Invoke-WebRequest 下载
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $MODEL_URL -OutFile $DOWNLOAD_FILE
    Write-Host "✓ 下载完成" -ForegroundColor Green
} catch {
    Write-Host "✗ 下载失败: $_" -ForegroundColor Red
    exit 1
}

# 解压模型
Write-Host ""
Write-Host "📦 开始解压..." -ForegroundColor Cyan

# 检查 7z 是否可用
$has7z = Get-Command 7z -ErrorAction SilentlyContinue
$hasTar = Get-Command tar -ErrorAction SilentlyContinue

if ($hasTar) {
    # 使用 tar (Windows 10+ 自带)
    tar xvf $DOWNLOAD_FILE -C $TARGET_DIR
} elseif ($has7z) {
    # 使用 7-Zip
    7z x $DOWNLOAD_FILE -o$TARGET_DIR
} else {
    Write-Host "✗ 错误: 未找到解压工具 (tar 或 7z)" -ForegroundColor Red
    Write-Host "  请安装 7-Zip 或使用 Windows 10+ (自带 tar)" -ForegroundColor Yellow
    exit 1
}

# 清理下载的压缩包
Write-Host ""
Write-Host "🧹 清理临时文件..." -ForegroundColor Cyan
Remove-Item $DOWNLOAD_FILE -Force

# 验证解压结果
if (Test-Path "$TARGET_DIR\$MODEL_NAME\encoder-epoch-20-avg-1.onnx") {
    Write-Host ""
    Write-Host "✓ 模型下载并解压成功!" -ForegroundColor Green
    Write-Host "  位置: $TARGET_DIR\$MODEL_NAME" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "现在可以运行: npm run tauri dev" -ForegroundColor Yellow
} else {
    Write-Host "✗ 解压可能失败，请检查文件" -ForegroundColor Red
    exit 1
}
