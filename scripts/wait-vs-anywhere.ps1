# wait-vs-anywhere.ps1 - poll for VS BuildTools link.exe
$log = "C:\TEMP\YunZaiInstall\install.log"
$msvcRoot = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC'

$deadline = (Get-Date).AddMinutes(30)
while ((Get-Date) -lt $deadline) {
    Start-Sleep -Seconds 15

    # Detect active installer processes
    $running = Get-Process -Name 'vs_setup','setup','Msiexec','Install*' -ErrorAction SilentlyContinue
    if ($running) {
        $names = ($running | ForEach-Object { "$($_.Name)[$($_.Id)]" }) -join ','
        $ts = Get-Date -Format 'HH:mm:ss'
        Add-Content -Path $log -Value "[$ts] running: $names" -Encoding UTF8
    }

    if (Test-Path $msvcRoot) {
        $vers = Get-ChildItem $msvcRoot -ErrorAction SilentlyContinue
        foreach ($v in $vers) {
            $le = Join-Path $msvcRoot "$v\bin\Hostx64\x64\link.exe"
            if (Test-Path $le) {
                $ts = Get-Date -Format 'HH:mm:ss'
                Add-Content -Path $log -Value "[$ts] DONE MSVC=$v link.exe=$le" -Encoding UTF8
                Write-Output "DONE MSVC=$v link.exe=$le"
                exit 0
            }
        }
    }
}
Add-Content -Path $log -Value "[$(Get-Date -Format 'HH:mm:ss')] TIMEOUT" -Encoding UTF8
exit 1
