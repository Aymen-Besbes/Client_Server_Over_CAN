#!/bin/bash
cargo run --bin server &
PID=$!
sleep 30
kill -9 $PID
echo "server killed"