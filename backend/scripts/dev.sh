#!/bin/zsh

cargo run --bin user &
USER_PID=$!

echo "Waiting for user..."
until nc -z localhost 50055; do
  sleep 1
done
echo "User is up!"

cargo run --bin gateway &
GATEWAY_PID=$!

wait $USER_PID $GATEWAY_PID
