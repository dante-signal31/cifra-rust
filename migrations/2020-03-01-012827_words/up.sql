-- Your SQL goes here
CREATE TABLE words(
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL,
    language_id INTEGER NOT NULL,
    FOREIGN KEY (language_id) REFERENCES languages(id)
        on delete cascade
        on update cascade
);