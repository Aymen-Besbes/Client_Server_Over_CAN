#!/bin/bash

# Check if an interface name is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <interface_name>"
    exit 1
fi

INTERFACE_NAME=$1

# Load the vcan kernel module
sudo modprobe vcan

# Setting up a Virtual CAN Interface

# Add a vcan interface
sudo ip link add dev "$INTERFACE_NAME" type vcan

# Bring the interface up
sudo ip link set "$INTERFACE_NAME" up

# Verify the interface
ip link show "$INTERFACE_NAME"