# TaTing model download script (PowerShell)

$ErrorActionPreference = "Stop"

Write-Host "=== TaTing Model Download Script ===" -ForegroundColor Cyan
Write-Host ""

# ── Helper ───────────────────────────────────────────────────────────────────

function Download-File($url, $out) {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $url -OutFile $out
}

function Extract-Tar($file, $dest) {
    $hasTar = Get-Command tar -ErrorAction SilentlyContinue
    $has7z  = Get-Command 7z  -ErrorAction SilentlyContinue
    if ($hasTar) {
        tar xvf $file -C $dest
    } elseif ($has7z) {
        7z x $file "-o$dest"
    } else {
        Write-Host "Error: no extraction tool found (install tar or 7-Zip)" -ForegroundColor Red
        exit 1
    }
}

# ── ASR model ────────────────────────────────────────────────────────────────

$ASR_NAME = "sherpa-onnx-zipformer-multi-zh-hans-2023-9-2"
$ASR_URL  = "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/${ASR_NAME}.tar.bz2"
$ASR_DIR  = "src-tauri\resources\models\sherpa-zh"

if (!(Test-Path $ASR_DIR)) {
    Write-Host "Error: directory not found: $ASR_DIR (run from project root)" -ForegroundColor Red
    exit 1
}

if (Test-Path "$ASR_DIR\$ASR_NAME") {
    Write-Host "ASR model already exists: $ASR_DIR\$ASR_NAME" -ForegroundColor Green
    $r = Read-Host "Re-download? (y/N)"
    if ($r -eq 'y' -or $r -eq 'Y') {
        Remove-Item -Path "$ASR_DIR\$ASR_NAME" -Recurse -Force
    } else {
        Write-Host "Skipping ASR model." -ForegroundColor Yellow
    }
}

if (!(Test-Path "$ASR_DIR\$ASR_NAME")) {
    Write-Host "Downloading ASR model (~100MB compressed)..." -ForegroundColor Cyan
    Download-File $ASR_URL "${ASR_NAME}.tar.bz2"
    Extract-Tar "${ASR_NAME}.tar.bz2" $ASR_DIR
    Remove-Item "${ASR_NAME}.tar.bz2" -Force
    if (Test-Path "$ASR_DIR\$ASR_NAME\encoder-epoch-20-avg-1.onnx") {
        Write-Host "ASR model downloaded: $ASR_DIR\$ASR_NAME" -ForegroundColor Green
    } else {
        Write-Host "Error: ASR model extraction failed" -ForegroundColor Red
        exit 1
    }
}

# ── Punctuation model ─────────────────────────────────────────────────────────

$PUNCT_NAME = "sherpa-onnx-punct-ct-transformer-zh-en-vocab272727-2024-04-12"
$PUNCT_URL  = "https://github.com/k2-fsa/sherpa-onnx/releases/download/punctuation-models/${PUNCT_NAME}.tar.bz2"
$PUNCT_DIR  = "src-tauri\resources\models\sherpa-punct"

New-Item -ItemType Directory -Force -Path $PUNCT_DIR | Out-Null

if (Test-Path "$PUNCT_DIR\$PUNCT_NAME") {
    Write-Host "Punctuation model already exists: $PUNCT_DIR\$PUNCT_NAME" -ForegroundColor Green
    $r = Read-Host "Re-download? (y/N)"
    if ($r -eq 'y' -or $r -eq 'Y') {
        Remove-Item -Path "$PUNCT_DIR\$PUNCT_NAME" -Recurse -Force
    } else {
        Write-Host "Skipping punctuation model." -ForegroundColor Yellow
    }
}

if (!(Test-Path "$PUNCT_DIR\$PUNCT_NAME")) {
    Write-Host "Downloading punctuation model (~60MB compressed)..." -ForegroundColor Cyan
    Download-File $PUNCT_URL "${PUNCT_NAME}.tar.bz2"
    Extract-Tar "${PUNCT_NAME}.tar.bz2" $PUNCT_DIR
    Remove-Item "${PUNCT_NAME}.tar.bz2" -Force
    if (Test-Path "$PUNCT_DIR\$PUNCT_NAME\model.onnx") {
        Write-Host "Punctuation model downloaded: $PUNCT_DIR\$PUNCT_NAME" -ForegroundColor Green
    } else {
        Write-Host "Error: punctuation model extraction failed" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "All models ready. Run: npm run tauri dev" -ForegroundColor Yellow
