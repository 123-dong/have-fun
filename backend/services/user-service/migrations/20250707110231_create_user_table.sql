-- Add migration script here
DROP SCHEMA IF EXISTS "user" CASCADE;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(25) NOT NULL,
    email TEXT UNIQUE NOT NULL
);