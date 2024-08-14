// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Int4,
        status -> Int2,
    }
}

diesel::table! {
    occupied_chessboard_square (id) {
        id -> Int4,
        game_id -> Int4,
        rank -> Int2,
        file -> Int2,
        chessboard_history_index -> Int2,
        piece_colour -> Int2,
        piece_type -> Int2,
    }
}

diesel::joinable!(occupied_chessboard_square -> game (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    occupied_chessboard_square,
);
