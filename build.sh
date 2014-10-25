#!/bin/bash

mkdir -p tmp/rivers

cargo build
PORT=3100 cargo run &
JOB=$!
sleep 1

cargo test && kill $JOB
