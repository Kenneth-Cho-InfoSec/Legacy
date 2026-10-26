param(
    [string]$Output = "$PSScriptRoot\..\analysis\bluetooth\windows-pnp-inventory.txt"
)

$ErrorActionPreference = "Stop"
$outDir = Split-Path -Parent $Output
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$sections = @()
$sections += "Generated: $(Get-Date -Format o)"
$sections += ""
$sections += "== Bluetooth class devices =="
$sections += (Get-PnpDevice -Class Bluetooth | Select-Object Status,Class,FriendlyName,InstanceId | Format-Table -AutoSize | Out-String)
$sections += ""
$sections += "== GM2/thinkplus/earbud related devices =="
$sections += (Get-PnpDevice | Where-Object { $_.FriendlyName -match 'GM2|thinkplus|Earbud|Headset|Bluetooth' } | Select-Object Status,Class,FriendlyName,InstanceId | Format-Table -AutoSize | Out-String)
$sections += ""
$sections += "== Bluetooth services =="
$sections += (Get-Service | Where-Object { $_.Name -match 'bth|bluetooth' -or $_.DisplayName -match 'Bluetooth' } | Select-Object Status,Name,DisplayName | Format-Table -AutoSize | Out-String)

$sections -join "`r`n" | Set-Content -Path $Output -Encoding UTF8
Write-Host "Wrote $Output"

