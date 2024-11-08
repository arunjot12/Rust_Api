-- Your SQL goes here
CREATE TABLE blocks (
    id SERIAL PRIMARY KEY,
    block_number INT NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)