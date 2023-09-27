-- Base app schema
-- User
CREATE TABLE "user" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    username VARCHAR(128) NOT NULL UNIQUE,
    -- Auth
    pwd VARCHAR(256),
    pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
    token_salt uuid NOT NULL DEFAULT gen_random_uuid()
);
-- Task
CREATE TABLE "task" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    title VARCHAR(256) NOT NULL
);