-- Your SQL goes here
CREATE TABLE vehicles (
    id SERIAL PRIMARY KEY,
    make VARCHAR NOT NULL,
    model VARCHAR NOT NULL,
    year INT NOT NULL
);

