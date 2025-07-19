-- Add migration script here
CREATE SCHEMA IF NOT EXISTS "user";
CREATE TABLE IF NOT EXISTS "user"."table_users" (
    pk_user_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username VARCHAR(20) NOT NULL
);