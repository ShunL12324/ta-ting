# Complete Process Tree Analysis for Port 5173

Write-Host "=== PORT 5173 DIAGNOSTIC REPORT ===" -ForegroundColor Cyan
Write-Host ""

# Get the process occupying port
$portInfo = netstat -ano | Select-String ":5173" | Select-String "LISTENING"
if (-not $portInfo) {
    Write-Host "Port 5173 is FREE" -ForegroundColor Green
    exit 0
}

$portInfo -match "\s+(\d+)$" | Out-Null
$leafPid = [int]$matches[1]

Write-Host "Port 5173 is occupied. Building process tree..." -ForegroundColor Yellow
Write-Host ""

# Build complete process tree
$processTree = @()
$currentPid = $leafPid

while ($currentPid) {
    $proc = Get-CimInstance Win32_Process -Filter "ProcessId = $currentPid"
    if (-not $proc) { break }

    $procInfo = Get-Process -Id $currentPid -ErrorAction SilentlyContinue

    $processTree += [PSCustomObject]@{
        PID = $currentPid
        Name = $proc.Name
        CommandLine = $proc.CommandLine
        StartTime = if ($procInfo) { $procInfo.StartTime } else { $null }
        ParentPID = $proc.ParentProcessId
    }

    $currentPid = $proc.ParentProcessId
}

# Display tree from root to leaf
Write-Host "PROCESS TREE (from parent to child):" -ForegroundColor White
Write-Host ""

$processTree = $processTree | Sort-Object @{Expression={$processTree.IndexOf($_)}; Descending=$true}

for ($i = 0; $i -lt $processTree.Count; $i++) {
    $p = $processTree[$i]
    $indent = "  " * ($processTree.Count - $i - 1)
    $arrow = if ($i -eq 0) { "" } else { "$indent`-- " }

    Write-Host "$arrow$($p.Name) (PID $($p.PID))" -ForegroundColor $(if ($i -eq $processTree.Count - 1) { "Yellow" } else { "Gray" })

    if ($p.CommandLine) {
        $cmd = $p.CommandLine
        if ($cmd.Length -gt 100) {
            $cmd = $cmd.Substring(0, 97) + "..."
        }
        Write-Host "$indent    CMD: $cmd" -ForegroundColor DarkGray
    }

    if ($p.StartTime) {
        Write-Host "$indent    Started: $($p.StartTime)" -ForegroundColor DarkGray
    }
    Write-Host ""
}

# Diagnosis
Write-Host "=== DIAGNOSIS ===" -ForegroundColor Cyan
Write-Host ""

$viteProc = $processTree | Where-Object { $_.CommandLine -like "*vite*" }
if ($viteProc) {
    Write-Host "ROOT CAUSE:" -ForegroundColor Red
    Write-Host "  A Vite development server (PID $($viteProc.PID)) is running" -ForegroundColor White
    Write-Host "  Started at: $($viteProc.StartTime)" -ForegroundColor White
    Write-Host ""
    Write-Host "LIKELY REASON:" -ForegroundColor Yellow
    Write-Host "  1. 'npm run dev' or 'npm run tauri:dev' was started previously" -ForegroundColor White
    Write-Host "  2. The terminal window was closed without stopping the server (Ctrl+C)" -ForegroundColor White
    Write-Host "  3. The process became orphaned and kept running in background" -ForegroundColor White
    Write-Host ""
    Write-Host "SOLUTION:" -ForegroundColor Green
    Write-Host "  Option 1 (Recommended):" -ForegroundColor Cyan
    Write-Host "    Stop-Process -Id $leafPid -Force" -ForegroundColor White
    Write-Host ""
    Write-Host "  Option 2 (Kill entire tree):" -ForegroundColor Cyan
    $rootPid = $processTree[0].PID
    Write-Host "    Stop-Process -Id $rootPid -Force" -ForegroundColor White
    Write-Host ""
    Write-Host "  Then restart:" -ForegroundColor Cyan
    Write-Host "    npm run tauri:dev" -ForegroundColor White
} else {
    Write-Host "UNEXPECTED:" -ForegroundColor Red
    Write-Host "  This is NOT a Vite server!" -ForegroundColor White
    Write-Host "  Some other application is using port 5173" -ForegroundColor White
    Write-Host "  You may need to change TaTing's dev port in vite.config.ts" -ForegroundColor White
}

Write-Host ""
Write-Host "================================" -ForegroundColor Cyan
