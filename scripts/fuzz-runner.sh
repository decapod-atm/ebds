#!/usr/bin/env bash

cargo install cargo-fuzz

cargo fuzz run -s none fuzz_build_message -- -runs=1000000 
cargo fuzz run -s none fuzz_omnibus_reply -- -runs=1000000 
cargo fuzz run -s none fuzz_omnibus_reply_from_bytes -- -runs=1000000 
