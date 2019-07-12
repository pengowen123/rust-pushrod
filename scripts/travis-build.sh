#!/usr/bin/env bash
#
# Build script for Travis CI

if [ "$TRAVIS_OS_NAME" = "osx" ]; then
  echo "Special build rules for OS X here"
  brew update
fi

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
  echo "Special build rules for Linux here"
  sudo apt update -y
fi

echo "Building Pushrod"

cargo build --tests
