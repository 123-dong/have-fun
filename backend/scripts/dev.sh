#!/bin/zsh

cargo run --bin user &
USER_PID=$!

echo "Waiting for user service..."
until nc -z localhost 50055; do
  sleep 1
done
echo "User service is up!"

cargo run --bin gateway &
GATEWAY_PID=$!

wait $USER_PID $GATEWAY_PID
