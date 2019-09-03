#!/usr/bin/env bash
#
# Build script for Travis CI

if [ "$TRAVIS_OS_NAME" = "osx" ]; then
  echo "Special build rules for OS X here"
fi

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
  sudo apt update -y
  sudo apt install gcc -y
  sudo apt install cmake -y
  sudo apt install libx11-dev -y
  sudo apt install xorg-dev libglu1-mesa-dev -y
fi

echo "Building Pushrod"

cargo build --tests

