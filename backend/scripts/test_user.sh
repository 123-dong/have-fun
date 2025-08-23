#!/bin/zsh
set -e

GRPC_HOST="localhost:50051"
SERVICE="user.v1.UserService"

# ANSI colors
BLUE='\033[1;34m'
GREEN='\033[1;32m'
RED='\033[1;31m'
NC='\033[0m' # No Color

RAND_SUFFIX=$(date +%s%N)
EMAIL="alice_${RAND_SUFFIX}@test.com"

echo "${BLUE}--- CREATE USER ---${NC}"
CREATE_RESP=$(grpcurl -plaintext -d "{\"name\":\"Alice\",\"email\":\"$EMAIL\"}" $GRPC_HOST $SERVICE/Create)
echo "$CREATE_RESP"
USER_ID=$(echo "$CREATE_RESP" | sed -n 's/.*"id": "\([^"]*\)".*/\1/p')
echo "${GREEN}Created user id: $USER_ID${NC}"

echo "${BLUE}--- GET USER ---${NC}"
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Get && \
echo "${GREEN}Get user success${NC}"

echo "${BLUE}--- UPDATE USER ---${NC}"
grpcurl -plaintext -d "{\"id\":\"$USER_ID\",\"name\":\"Alice Updated\",\"email\":\"$EMAIL\"}" $GRPC_HOST $SERVICE/Update && \
echo "${GREEN}Update user success${NC}"

echo "${BLUE}--- GET UPDATED USER ---${NC}"
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Get && \
echo "${GREEN}Get updated user success${NC}"

echo "${BLUE}--- DELETE USER ---${NC}"
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Delete && \
echo "${GREEN}Delete user success${NC}"

echo "${BLUE}--- GET DELETED USER (should fail) ---${NC}"
set +e
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Get
if [ $? -ne 0 ]; then
    echo "${RED}User not found (expected)${NC}"
fi
set -e

echo "${BLUE}--- LIST BULK USERS ---${NC}"
grpcurl -plaintext -d '{}' $GRPC_HOST $SERVICE/ListBulk && \
echo "${GREEN}ListBulk success${NC}"

echo "${BLUE}--- LIST FULL USERS (stream) ---${NC}"
grpcurl -plaintext -d '{}' $GRPC_HOST $SERVICE/ListFull && \
echo "${GREEN}ListFull success${NC}"
