# Run for 15 seconds and then kill it from another shell. Meanwhile, collect stats via `time -v`.
( /usr/bin/time -v ./target/release/snow-rs benchmark ) &
PID=$!
sleep 15
kill -SIGINT $PID
