#!/bin/bash
cargo build --release
kill -9 $(lsof -ti tcp:50018)
sleep 3
./target/release/alchemy-notify-to-discord >> release.log &
