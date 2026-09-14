# install-npm-deps.ps1 - 在 Windows 端装前端依赖
Set-Location E:\YunZai_APP
& 'C:\Program Files\nodejs\npm.cmd' install 2>&1 | Out-File -Encoding UTF8 E:\YunZai_APP\npm-install.log
Write-Output "exit: $LASTEXITCODE"
Get-ChildItem E:\YunZai_APP\node_modules\.bin | Select-Object -First 10 Name
