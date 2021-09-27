CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    post_type VARCHAR NOT NULL,
    CONTENT TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);