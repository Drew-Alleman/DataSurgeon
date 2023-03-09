#!/bin/bash

if [ -d "DataSurgeon" ]; then
    echo "[*] Removing old 'DataSurgeon' directory"
    rm -rf DataSurgeon 2>&1
fi

git clone https://github.com/Drew-Alleman/DataSurgeon
cd DataSurgeon
cargo build --release
if [ -f "$(pwd)/target/release/ds" ]; then
   chmod +x target/release/ds
    read -p "[!] DataSurgeon was succesfully compiled! Would you like to add it to your path? (requires password) [y/n]: " yn
    case $yn in
        [Yy]* ) sudo mv target/release/ds /usr/local/bin/ds;;
        [Nn]* ) echo "Done! ds can be found: /target/release/";;
        * ) echo "Please answer y or n.";;
    esac
    rm -rf DataSurgeon 2>&1
fi
