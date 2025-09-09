#!/bin/zsh
set -e

BASE_URL="http://localhost:3001/users"

# ANSI colors
BLUE='\033[1;34m'
GREEN='\033[1;32m'
RED='\033[1;31m'
NC='\033[0m'

log() { echo "\n${BLUE}% $1 ${NC}"; }

rand_str() {
  LC_ALL=C tr -dc 'a-z' </dev/urandom | head -c 6
}

NAME="User_$(rand_str)"
EMAIL="$(rand_str)@example.com"

log "CREATE USER"
CREATE_RESP=$(curl -s -w "\n" -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"$NAME\",\"email\":\"$EMAIL\"}")
echo "$CREATE_RESP"

USER_ID=$(echo "$CREATE_RESP" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')
echo "${GREEN}Created user ID: $USER_ID${NC}"

log "GET USER"
curl -s -w "\n" $BASE_URL/$USER_ID

NAME_UPDATE="Updated_$(rand_str)"
EMAIL_UPDATE="$(rand_str)@example.com"

log "UPDATE USER"
curl -s -w "\n" -X PUT $BASE_URL/$USER_ID \
  -H "Content-Type: application/json" \
  -d "{\"id\":\"$USER_ID\",\"name\":\"$NAME_UPDATE\",\"email\":\"$EMAIL_UPDATE\"}"

log "LIST USERS"
curl -s -w "\n" $BASE_URL

log "STREAM USERS"
curl -s -N $BASE_URL/stream | sed 's/^/  /'  # indent 

log "DELETE USER"
curl -s -w "\n" -X DELETE $BASE_URL/$USER_ID

log "GET USER AFTER DELETE (should fail)"
set +e
curl -s -w "\n" $BASE_URL/$USER_ID
set -e
