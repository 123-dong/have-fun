#!/usr/bin/env bash

BASE_URL="http://localhost:3000"

echo "=== Health Check ==="
curl -i "$BASE_URL/health"

echo -e "\n\n=== Create User ==="
CREATE_RESPONSE=$(curl -s -X POST "$BASE_URL/users" \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}')

echo "Response: $CREATE_RESPONSE"

USER_ID=$(echo "$CREATE_RESPONSE" | jq -r '.id')

if [ "$USER_ID" == "null" ] || [ -z "$USER_ID" ]; then
  echo "[X] Failed to create user."
  exit 1
fi

echo -e "\n=== Get User ==="
curl -i "$BASE_URL/users/$USER_ID"

echo -e "\n\n=== Update User ==="
curl -i -X PUT "$BASE_URL/users/$USER_ID" \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Updated", "email": "alice.updated@example.com"}'

echo -e "\n\n=== Get User Again ==="
curl -i "$BASE_URL/users/$USER_ID"

echo -e "\n\n=== Delete User ==="
curl -i -X DELETE "$BASE_URL/users/$USER_ID"

echo -e "\n\n=== Get Deleted User (Expect 404) ==="
curl -i "$BASE_URL/users/$USER_ID"
