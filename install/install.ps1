Start-Sleep -Milliseconds 1000

$executableDirectory = "C:/ds/"
$executablePath = "$executableDirectory" + "ds.exe";

if ((Test-Path -Path $executablePath -PathType Leaf)) {
  Write-Host "[*] Removing the previously installed version"
  Remove-Item -Path $executablePath -Force -ErrorAction SilentlyContinue | Out-Null
}

if ((Test-Path -Path ".\DataSurgeon" -PathType Container)) {
  Write-Host "[*] Removing the old 'DataSurgeon' directory"
  Remove-Item -Path "DataSurgeon" -Force -Recurse -ErrorAction SilentlyContinue | Out-Null
}

Write-Host "[*] Downloading DataSurgeon from Github..."
git clone https://github.com/Drew-Alleman/DataSurgeon/ --quiet
cd DataSurgeon
cargo build --release 

if (!(Test-Path -Path $executableDirectory -PathType Container)) {
  Write-Host "[*] Creating C:/ds/ to store the executable and plugin file"
  mkdir C:/ds/ | Out-Null
}

copy "$(Get-Location)\target\release\ds.exe" $executablePath
if (!(Test-Path -Path "$executableDirectory\plugins.json" -PathType Leaf)) {
  copy "$(Get-Location)\plugins.json" $executableDirectory
} else {
    Write-Host "[*] Existing plugins.json found, leaving it intact."
}

if ((Get-ItemProperty -Path 'Registry::HKEY_CURRENT_USER\Environment' -Name PATH -ErrorAction SilentlyContinue).Path -split ';' -notcontains $executableDirectory) {
    $yesOrNo = Read-Host "Would you like to add 'ds' to your PATH? This allows you to run the 'ds' command from any directory in your terminal. [Y/n]"
    if ($yesOrNo -eq 'Y' -or $yesOrNo -eq 'y') {
        Write-Host "[*] Binding ds.exe to user path (requires admin)"
        setx PATH "$env:PATH;$executableDirectory"
    } else {
        Write-Host "Skipped adding 'ds' to PATH."
    }
}

cd ..
