#!/bin/bash
cargo run --bin client >test1.log &
PID=$!
sleep 20
kill -9 $PID
echo "client killed"