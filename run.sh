#!/bin/bash
cargo run -- examples/test.c
nasm -felf64 output.asm -o output.o
ld output.o -o out
rm output.o

