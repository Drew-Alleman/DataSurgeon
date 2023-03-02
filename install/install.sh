sudo rm /usr/local/bin/ds 2>&1
sudo rm -rf DataSurgeon 2>&1
git clone https://github.com/Drew-Alleman/DataSurgeon 
cd DataSurgeon
cargo build --release
chmod +x target/release/ds
sudo mv target/release/ds /usr/local/bin/ds
sudo rm -rf DataSurgeon 2>&1
