#!/bin/bash
# TaTing model download script (Linux/macOS)

set -e

echo "=== TaTing Model Download Script ==="
echo ""

# ── Helper ──────────────────────────────────────────────────────────────────

download() {
    local url="$1"
    local out="$2"
    if command -v wget &> /dev/null; then
        wget -O "$out" "$url"
    elif command -v curl &> /dev/null; then
        curl -L -o "$out" "$url"
    else
        echo "Error: wget or curl not found"
        exit 1
    fi
}

# ── ASR model ────────────────────────────────────────────────────────────────

ASR_NAME="sherpa-onnx-zipformer-multi-zh-hans-2023-9-2"
ASR_URL="https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/${ASR_NAME}.tar.bz2"
ASR_DIR="src-tauri/resources/models/sherpa-zh"

if [ ! -d "$ASR_DIR" ]; then
    echo "Error: directory not found: $ASR_DIR (run from project root)"
    exit 1
fi

if [ -d "$ASR_DIR/$ASR_NAME" ]; then
    echo "ASR model already exists: $ASR_DIR/$ASR_NAME"
    read -p "Re-download? (y/N): " r
    if [ "$r" = "y" ] || [ "$r" = "Y" ]; then
        rm -rf "$ASR_DIR/$ASR_NAME"
    else
        echo "Skipping ASR model."
    fi
fi

if [ ! -d "$ASR_DIR/$ASR_NAME" ]; then
    echo "Downloading ASR model (~100MB compressed)..."
    download "$ASR_URL" "${ASR_NAME}.tar.bz2"
    tar xjf "${ASR_NAME}.tar.bz2" -C "$ASR_DIR"
    rm "${ASR_NAME}.tar.bz2"
    if [ -f "$ASR_DIR/$ASR_NAME/encoder-epoch-20-avg-1.onnx" ]; then
        echo "ASR model downloaded: $ASR_DIR/$ASR_NAME"
    else
        echo "Error: ASR model extraction failed"
        exit 1
    fi
fi

# ── Punctuation model ────────────────────────────────────────────────────────

PUNCT_NAME="sherpa-onnx-punct-ct-transformer-zh-en-vocab272727-2024-04-12"
PUNCT_URL="https://github.com/k2-fsa/sherpa-onnx/releases/download/punctuation-models/${PUNCT_NAME}.tar.bz2"
PUNCT_DIR="src-tauri/resources/models/sherpa-punct"

mkdir -p "$PUNCT_DIR"

if [ -d "$PUNCT_DIR/$PUNCT_NAME" ]; then
    echo "Punctuation model already exists: $PUNCT_DIR/$PUNCT_NAME"
    read -p "Re-download? (y/N): " r
    if [ "$r" = "y" ] || [ "$r" = "Y" ]; then
        rm -rf "$PUNCT_DIR/$PUNCT_NAME"
    else
        echo "Skipping punctuation model."
    fi
fi

if [ ! -d "$PUNCT_DIR/$PUNCT_NAME" ]; then
    echo "Downloading punctuation model (~60MB compressed)..."
    download "$PUNCT_URL" "${PUNCT_NAME}.tar.bz2"
    tar xjf "${PUNCT_NAME}.tar.bz2" -C "$PUNCT_DIR"
    rm "${PUNCT_NAME}.tar.bz2"
    if [ -f "$PUNCT_DIR/$PUNCT_NAME/model.onnx" ]; then
        echo "Punctuation model downloaded: $PUNCT_DIR/$PUNCT_NAME"
    else
        echo "Error: punctuation model extraction failed"
        exit 1
    fi
fi

echo ""
echo "All models ready. Run: npm run tauri dev"
