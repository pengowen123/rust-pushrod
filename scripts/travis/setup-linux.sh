#!/usr/bin/env bash
#
# Build script for Travis CI on Linux

echo "Installing RandR libraries for Travis-CI"
sudo apt-get install xorg-dev libglu1-mesa-dev

