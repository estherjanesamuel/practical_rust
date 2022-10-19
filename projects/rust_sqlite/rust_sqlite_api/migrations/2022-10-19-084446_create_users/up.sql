-- Your SQL goes here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    date_created TEXT NOT NULL
);

INSERT INTO
    "users"(name, address, date_created)
VALUES
("John", "123 Av Q", "Today");