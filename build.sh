#!/usr/bin/env bash

rustc ./dlhook.rs --crate-type cdylib
rustc ./dlhook.rs -C llvm-args=-x86-asm-syntax=intel --crate-type cdylib --emit asm
