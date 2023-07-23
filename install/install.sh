#!/bin/bash

# Sleep to let the web request finish
sleep 1
# Update OS and install OpenSSL libraries
echo "[*] Updating OS and installing OpenSSL libraries... (password required)"
sudo apt update && sudo apt upgrade && sudo apt install pkg-config libssl-dev

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
if git clone https://github.com/Drew-Alleman/DataSurgeon; then
    cd DataSurgeon || exit
    # Build the project
    cargo build --release
else
    echo "[!] Failed to download DataSurgeon from Github"
    exit 1
fi

# Check if the build succeeded
if [ -f "target/release/ds" ]; then
    if read -r -t 300 -p "Would you like to add 'ds' to your local bin? This will make 'ds' executable from any location in your terminal. (y/n) " response; then
        if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            echo "[*] Adding 'ds' to your local bin..."
            chmod +x target/release/ds
            sudo mv target/release/ds /usr/local/bin/
        else
            echo "Skipped adding 'ds' to local bin."
        fi
    else
        echo "Timed out. Skipped adding 'ds' to local bin."
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
