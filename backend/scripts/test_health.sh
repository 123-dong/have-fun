#!/usr/bin/env bash

set -e

echo "Sending HealthCheck request to gRPC server..."

grpcurl -plaintext \
  -import-path ./libs/proto \
  -proto health.proto \
  -d '{"name": "grpcurl"}' \
  [::1]:50051 \
  health.Health/HealthCheck
