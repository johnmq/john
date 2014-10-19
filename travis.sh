#!/bin/bash

cargo build
PORT=3100 cargo run &
sleep 1

cargo test
