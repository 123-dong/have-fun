#!/bin/bash

# Load environment variables
export $(grep -v '^#' .env | xargs)

SQL=$(cat <<EOF
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    email VARCHAR(100) UNIQUE
);

INSERT INTO users (name, email) VALUES
('Alice', 'alice@example.com'),
('Bob', 'bob@example.com');
EOF
)

docker exec -i rust_db psql -U $POSTGRES_USER -d $POSTGRES_DB -c "$SQL"

echo "Database reset and initialized with sample data."
