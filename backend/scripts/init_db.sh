#!/bin/bash

# Load environment variables
export $(grep -v '^#' .env | xargs)

SQL=$(cat <<EOF
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users CASCADE;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);

INSERT INTO users (name, email) VALUES
('Alice', 'alice@example.com'),
('Bob', 'bob@example.com');
EOF
)

docker exec -i rust_db psql -U $POSTGRES_USER -d $POSTGRES_DB -c "$SQL"

echo "@@@! Database reset and initialized with sample data."
