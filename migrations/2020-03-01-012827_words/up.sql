-- Your SQL goes here
CREATE TABLE words(
    id INTEGER NOT NULL PRIMARY KEY,
    word TEXT NOT NULL,
    word_pattern TEXT NOT NULL,
    language_id INTEGER NOT NULL,
    FOREIGN KEY (language_id) REFERENCES languages(id)
        on delete cascade
        on update cascade
);