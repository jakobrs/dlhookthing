#!/usr/bin/env bash

rustc ./dlhook.rs --crate-type cdylib -o dlhook.so "$@"
rustc ./dlhook.rs -C llvm-args=-x86-asm-syntax=intel --crate-type cdylib --emit asm "$@"
