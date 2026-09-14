# wait-vs.ps1 - 等 VS BuildTools link.exe 出现
$msvcRoot = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC'
$deadline = (Get-Date).AddMinutes(20)
while ((Get-Date) -lt $deadline) {
    Start-Sleep -Seconds 15
    if (Test-Path $msvcRoot) {
        $v = (Get-ChildItem $msvcRoot -ErrorAction SilentlyContinue | Select-Object -First 1).Name
        if ($v) {
            $linkExe = Join-Path $msvcRoot "$v\bin\Hostx64\x64\link.exe"
            if (Test-Path $linkExe) {
                $ts = Get-Date -Format 'HH:mm:ss'
                Write-Output "[$ts] DONE MSVC=$v link.exe=$linkExe"
                exit 0
            }
        }
    }
    $ts = Get-Date -Format 'HH:mm:ss'
    $exists = Test-Path $msvcRoot
    Write-Output "[$ts] Still installing... msvc_dir_exists=$exists"
}
Write-Output "TIMEOUT after 20 minutes"
exit 1
