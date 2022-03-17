-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    author VARCHAR(20) NOT NULL DEFAULT '[deleted]' REFERENCES users(username) ON DELETE SET DEFAULT,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN DEFAULT 'f',
    timestamp TIMESTAMP(1) NOT NULL
);