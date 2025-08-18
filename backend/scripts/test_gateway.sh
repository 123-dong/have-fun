#!/usr/bin/env bash
set -euo pipefail

GATEWAY_URL="http://localhost:3000/users"

# Hàm lấy id từ JSON trả về
extract_id() {
  echo "$1" | awk -F'"id":' '{print $2}' | awk -F'"' '{print $2}'
}

echo "=== CREATE USER ==="
CREATE_RESP=$(curl -s -X POST "$GATEWAY_URL/create" \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@test.com"}')

echo "$CREATE_RESP"
USER_ID=$(extract_id "$CREATE_RESP")

if [ -z "$USER_ID" ]; then
  echo "Failed to extract user id. Make sure the gateway is running and returns JSON with 'id'."
  exit 1
fi
echo "Created user id: $USER_ID"
echo

echo "=== GET USER ==="
curl -s -X POST "$GATEWAY_URL/get" \
  -H "Content-Type: application/json" \
  -d "{\"id\":\"$USER_ID\"}"
echo -e "\n"

echo "=== UPDATE USER ==="
curl -s -X POST "$GATEWAY_URL/update" \
  -H "Content-Type: application/json" \
  -d "{\"id\":\"$USER_ID\",\"name\":\"Alice Updated\",\"email\":\"alice_updated@test.com\"}"
echo -e "\n"

echo "=== GET UPDATED USER ==="
curl -s -X POST "$GATEWAY_URL/get" \
  -H "Content-Type: application/json" \
  -d "{\"id\":\"$USER_ID\"}"
echo -e "\n"

echo "=== LIST USERS ==="
curl -s -X POST "$GATEWAY_URL/list" -H "Content-Type: application/json"
echo -e "\n"

echo "=== DELETE USER ==="
curl -s -X POST "$GATEWAY_URL/delete" \
  -H "Content-Type: application/json" \
  -d "{\"id\":\"$USER_ID\"}"
echo -e "\n"

echo "=== GET DELETED USER (should fail) ==="
curl -s -X POST "$GATEWAY_URL/get" \
  -H "Content-Type: application/json" \
  -d "{\"id\":\"$USER_ID\"}" || echo "User not found (expected)"
echo
