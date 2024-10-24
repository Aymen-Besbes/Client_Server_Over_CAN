#!/bin/bash
cargo run --bin server &
PID=$!
sleep 20
kill -9 $PID