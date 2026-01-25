#!/bin/bash
# TaTing 模型下载脚本 (Linux/macOS)

set -e

echo "=== TaTing 模型下载脚本 ==="
echo ""

MODEL_NAME="sherpa-onnx-zipformer-multi-zh-hans-2023-9-2"
MODEL_URL="https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/${MODEL_NAME}.tar.bz2"
TARGET_DIR="src-tauri/resources/models/sherpa-zh"
DOWNLOAD_FILE="${MODEL_NAME}.tar.bz2"

# 检查目标目录
if [ ! -d "$TARGET_DIR" ]; then
    echo "✗ 错误: 目标目录不存在: $TARGET_DIR"
    echo "  请确保在项目根目录运行此脚本"
    exit 1
fi

# 检查模型是否已存在
if [ -d "$TARGET_DIR/$MODEL_NAME" ]; then
    echo "✓ 模型已存在: $TARGET_DIR/$MODEL_NAME"
    read -p "是否重新下载? (y/N): " response
    if [ "$response" != "y" ] && [ "$response" != "Y" ]; then
        echo "跳过下载"
        exit 0
    fi
    rm -rf "$TARGET_DIR/$MODEL_NAME"
fi

# 下载模型
echo "📥 开始下载模型..."
echo "   URL: $MODEL_URL"
echo "   大小: 约 100MB (压缩后)"
echo ""

if command -v wget &> /dev/null; then
    wget -O "$DOWNLOAD_FILE" "$MODEL_URL"
elif command -v curl &> /dev/null; then
    curl -L -o "$DOWNLOAD_FILE" "$MODEL_URL"
else
    echo "✗ 错误: 未找到 wget 或 curl"
    echo "  请安装其中一个下载工具"
    exit 1
fi

echo "✓ 下载完成"

# 解压模型
echo ""
echo "📦 开始解压..."
tar xjf "$DOWNLOAD_FILE" -C "$TARGET_DIR"

# 清理下载的压缩包
echo ""
echo "🧹 清理临时文件..."
rm "$DOWNLOAD_FILE"

# 验证解压结果
if [ -f "$TARGET_DIR/$MODEL_NAME/encoder-epoch-20-avg-1.onnx" ]; then
    echo ""
    echo "✓ 模型下载并解压成功!"
    echo "  位置: $TARGET_DIR/$MODEL_NAME"
    echo ""
    echo "现在可以运行: npm run tauri dev"
else
    echo "✗ 解压可能失败，请检查文件"
    exit 1
fi
