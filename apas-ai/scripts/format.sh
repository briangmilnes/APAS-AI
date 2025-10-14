#!/bin/bash
# Format all Rust source files with nightly rustfmt

rustfmt +nightly -v src/*.rs src/*/*.rs tests/*.rs tests/*/*.rs benches/*.rs benches/*/*.rs

