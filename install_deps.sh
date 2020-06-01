#!/bin/sh

rustup component add llvm-tools-preview && cargo install cargo-xbuild bootimage
