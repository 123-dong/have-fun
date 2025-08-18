#!/bin/zsh
set -e

GRPC_HOST="localhost:50051"
SERVICE="user.v1.UserService"

RAND_SUFFIX=$(date +%s%N)
EMAIL="alice_${RAND_SUFFIX}@test.com"

echo "=== CREATE USER ==="
CREATE_RESP=$(grpcurl -plaintext -d "{\"name\":\"Alice\",\"email\":\"$EMAIL\"}" $GRPC_HOST $SERVICE/Create)
echo "$CREATE_RESP"

USER_ID=$(echo "$CREATE_RESP" | sed -n 's/.*"id": "\([^"]*\)".*/\1/p')
echo "Created user id: $USER_ID"

echo "=== GET USER ==="
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Get

echo "=== UPDATE USER ==="
grpcurl -plaintext -d "{\"id\":\"$USER_ID\",\"name\":\"Alice Updated\",\"email\":\"$EMAIL\"}" $GRPC_HOST $SERVICE/Update

echo "=== GET UPDATED USER ==="
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Get

echo "=== DELETE USER ==="
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Delete

echo "=== GET DELETED USER (should fail) ==="
set +e
grpcurl -plaintext -d "{\"id\":\"$USER_ID\"}" $GRPC_HOST $SERVICE/Get
if [ $? -ne 0 ]; then
    echo "User not found (expected)"
fi
set -e
