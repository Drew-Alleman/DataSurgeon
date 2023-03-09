$executablePath = "C:/ds/ds.exe";
if ((Test-Path -Path $executablePath -PathType Leaf)) {
  Write-Host "[*] Removing the previously installed version"
  Remove-Item -Path $executablePath -Force -ErrorAction SilentlyContinue | Out-Null

}

if ((Test-Path -Path "DataSurgeon" -PathType Leaf)) {
  Write-Host "[*] Removing the old 'DataSurgeon' directory"
  Remove-Item -Path "DataSurgeon" -Force -ErrorAction SilentlyContinue | Out-Null
}
Write-Host -Message "[*] Downloading DataSurgeon from Github..."
git clone https://github.com/Drew-Alleman/DataSurgeon/ --quiet
cd DataSurgeon
cargo build --release 
if (-not(Test-Path -Path "C:/ds/" -PathType Leaf)) {
  Write-Host "[*] Creating C:/ds/ to store the executable"
  mkdir C:/ds/ | Out-Null
}
Write-Host "[*] Binding ds.exe to path (requires admin)"
copy "$(Get-Location)\target\release\ds.exe" "C:/ds/ds.exe"
setx PATH "$env:PATH;C:/ds/"
