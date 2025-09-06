#!/bin/zsh
set -e

BASE_URL="http://localhost:3001/users"

# ANSI colors
BLUE='\033[1;34m'
GREEN='\033[1;32m'
RED='\033[1;31m'
NC='\033[0m' # No Color

rand_str() {
  LC_ALL=C tr -dc 'a-z' </dev/urandom | head -c 6
}

NAME="User_$(rand_str)"
EMAIL="$(rand_str)@example.com"

echo "${BLUE}=== CREATE USER ===${NC}"
CREATE_RESP=$(curl -s -X POST $BASE_URL \
    -H "Content-Type: application/json" \
    -d "{\"name\":\"$NAME\",\"email\":\"$EMAIL\"}")
echo "$CREATE_RESP"

# Lấy user ID
USER_ID=$(echo "$CREATE_RESP" | grep -o '"id":"[^"]*"' | head -1 | sed 's/"id":"\(.*\)"/\1/')
echo "${GREEN}Created user ID: $USER_ID${NC}"

echo "${BLUE}=== GET USER ===${NC}"
curl -s $BASE_URL/$USER_ID && echo "${GREEN}Get user success${NC}"

# Random update name/email
NAME_UPDATE="Updated_$(rand_str)"
EMAIL_UPDATE="$(rand_str)@example.com"

echo "${BLUE}=== UPDATE USER ===${NC}"
curl -s -X PUT $BASE_URL/$USER_ID \
    -H "Content-Type: application/json" \
    -d "{\"name\":\"$NAME_UPDATE\",\"email\":\"$EMAIL_UPDATE\"}" && echo "${GREEN}Update user success${NC}"

echo "${BLUE}=== GET USER AFTER UPDATE ===${NC}"
curl -s $BASE_URL/$USER_ID && echo "${GREEN}Get updated user success${NC}"

echo "${BLUE}=== LIST BULK USERS ===${NC}"
curl -s $BASE_URL/bulk && echo "${GREEN}ListBulk success${NC}"

echo "${BLUE}=== LIST FULL USERS ===${NC}"
curl -s $BASE_URL/full && echo "${GREEN}ListFull success${NC}"

echo "${BLUE}=== DELETE USER ===${NC}"
curl -s -X DELETE $BASE_URL/$USER_ID && echo "${GREEN}Delete user success${NC}"

echo "${BLUE}=== GET USER AFTER DELETE (should fail) ===${NC}"
set +e
curl -s $BASE_URL/$USER_ID
if [ $? -ne 0 ]; then
    echo "${RED}User not found (fail expected)${NC}"
fi
set -e
