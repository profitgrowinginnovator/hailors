#!/bin/bash

sudo apt update && sudo apt -y full-upgrade
sudo apt install -y imagemagick
sudo apt install -y imx500-all
sudo apt install -y  rpi-eeprom
sudo rpi-eeprom-update -a
# Path to the config.txt file
CONFIG_FILE="/boot/firmware/config.txt"

# Parameter to enable PCIe Gen 3.0
PARAMETER="dtparam=pciex1_gen=3"

# Check if the parameter is already present
if grep -q "^$PARAMETER" "$CONFIG_FILE"; then
    echo "Parameter '$PARAMETER' is already set in $CONFIG_FILE."
else
    # Add the parameter to the config.txt file
    echo "Adding parameter '$PARAMETER' to $CONFIG_FILE..."
    echo "$PARAMETER" | sudo tee -a "$CONFIG_FILE" > /dev/null

    if [ $? -eq 0 ]; then
        echo "Parameter added successfully."
    else
        echo "Failed to add parameter to $CONFIG_FILE. Please check permissions."
        exit 1
    fi
fi
sudo apt install -y hailo-all


# Enable VNC
echo "Enabling VNC..."
sudo raspi-config nonint do_vnc 0
if [ $? -eq 0 ]; then
    echo "VNC enabled successfully."
else
    echo "Failed to enable VNC. Please check manually."
    exit 1
fi
# Install Visual Studio Code
echo "Installing Visual Studio Code..."
wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > microsoft.gpg
sudo install -o root -g root -m 644 microsoft.gpg /usr/share/keyrings/
rm microsoft.gpg
echo "deb [arch=arm64,armhf signed-by=/usr/share/keyrings/microsoft.gpg] https://packages.microsoft.com/repos/code stable main" | sudo tee /etc/apt/sources.list.d/vscode.list
sudo apt-get update -y
sudo apt-get install -y code
if [ $? -eq 0 ]; then
    echo "Visual Studio Code installed successfully."
else
    echo "Failed to install Visual Studio Code."
    exit 1
fi

# Install Rust
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
if [ $? -eq 0 ]; then
    echo "Rust installed successfully."
else
    echo "Failed to install Rust."
    exit 1
fi

# Install C++ and C development libraries
echo "Installing C++ and C development libraries..."
sudo apt-get install -y build-essential g++ gcc cmake make libtool autoconf automake pkg-config lldb gdb
if [ $? -eq 0 ]; then
    echo "C++ and C development libraries installed successfully."
else
    echo "Failed to install C++ and C development libraries."
    exit 1
fi

# install code extensions
# Install Rust extensions
code --install-extension rust-lang.rust-analyzer

# Install C++ development extensions
code --install-extension ms-vscode.cpptools
code --install-extension ms-vscode.cpptools-extension-pack
code --install-extension ms-vscode.cmake-tools
code --install-extension twxs.cmake

# Debugging extensions
code --install-extension ms-vscode.cpptools
code --install-extension vadimcn.vscode-lldb

# Optional: Helpful extensions for Rust/C++ development
code --install-extension ms-vscode.cmake-tools
code --install-extension eamodio.gitlens

# Prompt for reboot
read -p "Settings updated. Do you want to reboot now? (y/n): " REBOOT
if [[ "$REBOOT" =~ ^[Yy]$ ]]; then
    echo "Rebooting..."
    sudo reboot
else
    echo "Reboot canceled. Please reboot manually for the changes to take effect."
fi
