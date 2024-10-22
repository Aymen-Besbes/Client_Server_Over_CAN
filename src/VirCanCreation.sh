#!/bin/bash
#Load the vcan kernel module
sudo modprobe vcan

#Setting up a Virtual CAN Interface

#Add a vcan interface
sudo ip link add dev vcan0 type vcan

#bring the interface up
sudo ip link set up vcan0

#verify the interface
ip link show vcan0