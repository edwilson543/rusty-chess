CREATE TABLE game (
    id SERIAL PRIMARY KEY,
    status TEXT NOT NULL
);

CREATE TABLE chessboard_square (
    id SERIAL PRIMARY KEY,
    game_id INTEGER NOT NULL REFERENCES game(id),
    rank SMALLINT NOT NULL CHECK(rank >= 1 AND rank <= 8),
    file SMALLINT NOT NULL CHECK(file >= 1 AND file <= 8),
    max_sequence_index SMALLINT NOT NULL,
    piece_type TEXT,
    piece_colour TEXT
);
