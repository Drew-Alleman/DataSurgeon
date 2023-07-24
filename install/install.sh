#!/bin/bash

# Sleep to let the web request finish
sleep 1
# Update OS and install OpenSSL libraries
echo "[*] Updating OS and installing OpenSSL libraries... (password required)"
sudo apt update > /dev/null && sudo apt upgrade > /dev/null && sudo apt install pkg-config libssl-dev > /dev/null

# Check and remove existing DataSurgeon directory if it exists
if [ -d "DataSurgeon" ]; then
    echo "[*] Removing old 'DataSurgeon' directory..."
    rm -rf DataSurgeon
fi

# Check and remove existing 'ds' executable if it exists
if [ -f "/usr/local/bin/ds" ]; then
    echo "[*] Removing old 'ds' executable..."
    sudo rm /usr/local/bin/ds
fi

# Clone DataSurgeon's source from Github
echo "[*] Downloading DataSurgeon's source from Github..."
if git clone https://github.com/Drew-Alleman/DataSurgeon > /dev/null 2>&1; then
    cd DataSurgeon || exit
    # Build the project
    echo "[*] Building the project..."
    if cargo build --release > /dev/null 2>&1; then
        echo "[*] Build succeeded."
    else
        echo "[!] Build failed."
        exit 1
    fi
else
    echo "[!] Failed to download DataSurgeon from Github"
    exit 1
fi

# Check if the build succeeded
if [ -f "target/release/ds" ]; then
    if [[ "$1" =~ ^([yY][eE][sS]|[yY])$ ]]; then        
        echo "[*] Adding 'ds' to your local bin..."
        chmod +x target/release/ds
        sudo mv target/release/ds /usr/local/bin/
    else
        echo "Skipped adding 'ds' to local bin."
    fi
else
    echo "[!] Build failed. The 'ds' executable does not exist."
    exit 1
fi

# Check for existing plugins.json and only move the new file if it doesn't exist
mkdir -p ~/.DataSurgeon
if [ ! -f ~/.DataSurgeon/plugins.json ]; then
    mv plugins.json ~/.DataSurgeon/
    echo "[*] Moved plugins.json to ~/.DataSurgeon/"
else
    echo "[*] Existing plugins.json found, leaving it intact."
fi

# Clean up
cd ..
