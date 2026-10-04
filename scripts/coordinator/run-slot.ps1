param(
    [Parameter(Mandatory = $true)][int]$Slot,
    [ValidateSet("n", "s", "b", "mix")][string]$Kind = "mix",
    [int]$StartWindowMinutes = 85
)
# A production slot: runs production lanes one after another, asking for the next batch each time.
# Kinds: n = native handlers, s = small functions, b = functions of 250 bytes and more, mix = take turns.
# It stops starting new lanes after $StartWindowMinutes so the last lane can finish before the
# two-hour limit on background tasks, when a STOP file exists, or when the queue is empty.
# Before each lane it waits until there is room: at least 5 GB of free memory AND at least 14 GB of
# commit headroom. On 2026-10-04 the machine ran out of commit (not free memory) at 74 lanes.
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$root = Split-Path -Parent (Split-Path -Parent $here)
$coord = "$root\.artifacts\scratch\coordinator"
$py = "$root\.venv\Scripts\python.exe"
$counts = @{ n = @(1, 0, 0); s = @(0, 1, 0); b = @(0, 0, 1) }
$turns = @("n", "s", "b")
$started = Get-Date
$ran = 0
while ($true) {
    if (Test-Path "$coord\STOP") { "slot ${Slot}: stop file present"; break }
    if (((Get-Date) - $started).TotalMinutes -gt $StartWindowMinutes) { "slot ${Slot}: start window closed"; break }
    $os = Get-CimInstance Win32_OperatingSystem
    $free = $os.FreePhysicalMemory / 1MB
    $headroom = $os.FreeVirtualMemory / 1MB
    if ($free -lt 5 -or $headroom -lt 14) {
        "slot ${Slot}: {0:N1} GB free, {1:N1} GB commit headroom: waiting" -f $free, $headroom
        Start-Sleep -Seconds (90 + (Get-Random -Maximum 60))
        continue
    }
    $elapsed = ((Get-Date) - $started).TotalMinutes
    $want = if ($Kind -eq "mix") { $turns[($ran + $Slot) % 3] } else { $Kind }
    # Large-function lanes can run for well over an hour. Starting one late means the two-hour task limit
    # kills it mid-work, so a slot only starts one in its first 15 minutes; later it takes quick batches,
    # and those not after 65 minutes.
    # Measured since: large-function lanes mostly take 30 to 60 minutes, quick batches about 30, and a lane
    # outlives its slot if the slot is stopped. So: large lanes start up to 45 minutes in, quick ones up to 85.
    if ($want -eq "b" -and $elapsed -gt 45) {
        if ($Kind -eq "b") { "slot ${Slot}: too late to start another large-function lane"; break }
        $want = "s"
    }
    if ($elapsed -gt 85) { "slot ${Slot}: start window closed"; break }
    $lane = $null
    foreach ($k in @($want) + ($turns | Where-Object { $_ -ne $want })) {
        $c = $counts[$k]
        $out = & $py "$here\make_briefs_prod.py" $c[0] $c[1] $c[2]
        $match = $out | Select-String -Pattern '^LANES (\S+)'
        if ($match) { $lane = $match.Matches[0].Groups[1].Value; break }
    }
    if (-not $lane) { "slot ${Slot}: queue is empty"; break }
    "slot ${Slot}: starting $lane"
    & "$here\run-lane.ps1" -Lane $lane
    $ran++
    Start-Sleep -Seconds (Get-Random -Maximum 20)
}
"SLOT $Slot DONE after $ran lanes"
