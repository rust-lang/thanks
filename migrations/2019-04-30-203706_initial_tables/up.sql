CREATE TABLE commits (
    sha VARCHAR NOT NULL PRIMARY KEY,
    author VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);
CREATE TABLE repositories (
    name VARCHAR NOT NULL PRIMARY KEY,
    latest_commit VARCHAR
);
