Remove-Item -Path DataSurgeon -Force -ErrorAction SilentlyContinue | Out-Null
Remove-Item -Path "C:/ds/ds.exe" -Force -ErrorAction SilentlyContinue | Out-Null

git clone https://github.com/Drew-Alleman/DataSurgeon/
cd DataSurgeon
cargo build --release
mkdir C:/ds/
copy "$(Get-Location)\target\release\ds.exe" "C:/ds/ds.exe"
setx PATH "$env:PATH;C:/ds/"
