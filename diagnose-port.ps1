# Port 5173 Diagnostic Script

Write-Host "Checking port 5173..." -ForegroundColor Cyan
Write-Host ""

$port = 5173

# Find process using port
$connections = netstat -ano | Select-String ":$port\s" | Select-String "LISTENING"

if (-not $connections) {
    Write-Host "Port $port is FREE" -ForegroundColor Green
    exit 0
}

# Extract PID
$connections | ForEach-Object {
    if ($_.Line -match "\s+(\d+)$") {
        $pid = [int]$matches[1]
        if ($pid -eq 0) { return }

        Write-Host "Port is OCCUPIED by PID: $pid" -ForegroundColor Yellow
        Write-Host ""

        # Get process info
        $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
        if ($process) {
            Write-Host "Process Name: $($process.ProcessName)" -ForegroundColor White
            Write-Host "Process ID:   $($process.Id)" -ForegroundColor White
            Write-Host "Start Time:   $($process.StartTime)" -ForegroundColor White
            Write-Host "Path:         $($process.Path)" -ForegroundColor White
            Write-Host ""

            # Get command line
            $wmiProcess = Get-CimInstance Win32_Process -Filter "ProcessId = $pid"
            if ($wmiProcess) {
                Write-Host "Command Line:" -ForegroundColor White
                Write-Host "  $($wmiProcess.CommandLine)" -ForegroundColor Gray
                Write-Host ""

                # Check if Vite
                if ($wmiProcess.CommandLine -like "*vite*") {
                    Write-Host "DIAGNOSIS: This is a Vite dev server" -ForegroundColor Yellow
                    Write-Host "REASON: Previous TaTing dev server was not properly closed" -ForegroundColor Yellow
                    Write-Host ""
                    Write-Host "SOLUTION:" -ForegroundColor Cyan
                    Write-Host "  Run: Stop-Process -Id $pid -Force" -ForegroundColor White
                    Write-Host "  Then: npm run tauri:dev" -ForegroundColor White
                } else {
                    Write-Host "DIAGNOSIS: NOT a Vite server, some other app" -ForegroundColor Red
                }

                # Show process tree
                Write-Host ""
                Write-Host "Process Tree:" -ForegroundColor White
                $currentPid = $pid
                $depth = 0

                while ($currentPid -and $depth -lt 5) {
                    $currentProc = Get-CimInstance Win32_Process -Filter "ProcessId = $currentPid"
                    if (-not $currentProc) { break }

                    $procInfo = Get-Process -Id $currentPid -ErrorAction SilentlyContinue
                    $procName = if ($procInfo) { $procInfo.ProcessName } else { "Unknown" }

                    $indent = "  " * $depth
                    Write-Host "$indent$procName (PID $currentPid)" -ForegroundColor Gray

                    $currentPid = $currentProc.ParentProcessId
                    $depth++
                }
            }
        }
    }
}

Write-Host ""
