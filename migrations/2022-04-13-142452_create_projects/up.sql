-- Your SQL goes here
CREATE TABLE projects(
    title VARCHAR(30) PRIMARY KEY,
    short_desc VARCHAR(100) NOT NULL,
    long_desc TEXT NOT NULL,
    repo VARCHAR NOT NULL
);