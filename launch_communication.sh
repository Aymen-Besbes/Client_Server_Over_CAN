#!/bin/bash
if [ $# -ne 2]; then
    if [ $# -ne 1];then
        echo "invalid command please check it should be at least with./script application"
        exit
    fi    
    echo "Usage: $0 <default config path>"
    path= [src/config/default.json]
else
        path=$2
fi

echo $path
cargo run --bin $1 $path