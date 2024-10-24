#!/bin/bash
cargo run --bin client &
PID=$!
sleep 20
kill -9 $PID