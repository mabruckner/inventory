-- Your SQL goes here

CREATE TABLE classes (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    unit VARCHAR NOT NULL,
    schema VARCHAR NOT NULL
);

CREATE TABLE batches (
    id INTEGER NOT NULL PRIMARY KEY,
    class INTEGER NOT NULL,
    quantity VARCHAR NOT NULL,
    data VARCHAR NOT NULL,
    FOREIGN KEY (class) REFERENCES classes(id)
);
