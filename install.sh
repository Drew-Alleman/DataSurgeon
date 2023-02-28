git clone https://github.com/Drew-Alleman/DataSurgeon 
cd DataSurgeon
cargo build --release
chmod +x target/release/ds
sudo mv target/release/ds /usr/local/bin/ds
