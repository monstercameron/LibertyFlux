param(
    [int]$Target = 44,
    [int]$Minutes = 112,
    [int]$TickEverySeconds = 300
)
# Supervisor: keeps the lane count at $Target and runs the five-minute tick, so the coordinator model is
# not woken for every lane that ends. Runs as ONE background task; it is stopped by the two-hour limit on
# background tasks, so it exits by itself after $Minutes and the coordinator starts a fresh one.
# Lanes it starts are separate detached processes and keep running after it exits or is stopped.
# Stop starting lanes: create the file STOP beside supervisor.json. Log: .artifacts\logs\supervisor.log
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$root = Split-Path -Parent (Split-Path -Parent $here)
$coord = "$root\.artifacts\scratch\coordinator"
$py = "$root\.venv\Scripts\python.exe"
$log = "$root\.artifacts\logs\supervisor.log"
$counts = @{ n = @(1, 0, 0); s = @(0, 1, 0); b = @(0, 0, 1) }
$cycle = @("s", "b", "s", "b", "b", "n")
$started = Get-Date
$lastTick = (Get-Date).AddSeconds(-$TickEverySeconds)
$launched = 0
$turn = 0

function Say([string]$text) {
    $line = "{0} {1}" -f (Get-Date -Format "HH:mm:ss"), $text
    Add-Content -Path $log -Value $line
    $line
}

Say "supervisor started: target $Target lanes, runs $Minutes minutes"
while (((Get-Date) - $started).TotalMinutes -lt $Minutes) {
    if (((Get-Date) - $lastTick).TotalSeconds -ge $TickEverySeconds) {
        $lastTick = Get-Date
        $tick = & $py "$here\tick.py" 0 2>&1 | Select-Object -Last 1
        Say "tick: $tick"
    }
    $stop = Test-Path "$coord\STOP"
    # Settings are read on every pass from supervisor.json, so the hourly review can retune a running
    # supervisor: target (lanes), min_free_gb, min_headroom_gb, cycle (lane kinds in order).
    $minFree = 5.5; $minHeadroom = 15
    if (Test-Path "$coord\supervisor.json") {
        try {
            $cfg = Get-Content "$coord\supervisor.json" -Raw | ConvertFrom-Json
            if ($cfg.target) { $Target = [int]$cfg.target }
            if ($cfg.min_free_gb) { $minFree = [double]$cfg.min_free_gb }
            if ($cfg.min_headroom_gb) { $minHeadroom = [double]$cfg.min_headroom_gb }
            if ($cfg.cycle) { $cycle = @($cfg.cycle) }
        } catch { Say "supervisor.json unreadable, keeping current settings" }
    }
    $lanes = @(Get-CimInstance Win32_Process -Filter "Name like 'muse%'").Count
    $os = Get-CimInstance Win32_OperatingSystem
    $free = $os.FreePhysicalMemory / 1MB
    $headroom = $os.FreeVirtualMemory / 1MB
    $room = ($free -ge $minFree) -and ($headroom -ge $minHeadroom)
    $startedNow = @()
    if (-not $stop -and $room -and $lanes -lt $Target) {
        # At most two per pass, so memory is read again before the next ones.
        foreach ($i in 1..([Math]::Min(2, $Target - $lanes))) {
            $want = $cycle[$turn % $cycle.Count]
            $turn++
            $lane = $null
            foreach ($k in @($want) + (@("s", "b", "n") | Where-Object { $_ -ne $want })) {
                if ($k -eq "m") {
                    # A naming lane: names only, no checker and no builds.
                    $out = & $py "$here\make_briefs_names.py" 1 150
                    $match = $out | Select-String -Pattern '^LANES (\S+)'
                    if ($match) { $lane = $match.Matches[0].Groups[1].Value; break }
                    continue
                }
                $c = $counts[$k]
                $out = & $py "$here\make_briefs_prod.py" $c[0] $c[1] $c[2]
                $match = $out | Select-String -Pattern '^LANES (\S+)'
                if ($match) { $lane = $match.Matches[0].Groups[1].Value; break }
            }
            if (-not $lane) { Say "queue is empty"; break }
            # Detached through start_lane.py (its own process group, no console), so lanes outlive the supervisor.
            & $py "$here\start_lane.py" $lane | Out-Null
            $launched++
            $startedNow += $lane
            Start-Sleep -Seconds 8
        }
    }
    $state = if ($stop) { "STOP file present" } elseif (-not $room) { "waiting for memory" } elseif ($startedNow) { "started " + ($startedNow -join ", ") } else { "at target" }
    Say ("lanes {0}/{1}, free {2:N1} GB, commit headroom {3:N1} GB, {4}; started so far {5}; {6} min left" -f `
        $lanes, $Target, $free, $headroom, $state, $launched, [int]($Minutes - ((Get-Date) - $started).TotalMinutes))
    Start-Sleep -Seconds 45
}
Say "supervisor finished its window: started $launched lanes"
"SUPERVISOR DONE"
