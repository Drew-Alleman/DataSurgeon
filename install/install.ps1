git clone https://github.com/Drew-Alleman/DataSurgeon/
cd DataSurgeon
cargo build --release
mkdir C:/ds/
copy "$(Get-Location)\target\release\ds.exe" "C:/ds/ds.exe"
setx PATH "$env:PATH;C:/ds/"
