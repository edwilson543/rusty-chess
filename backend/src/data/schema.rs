// @generated automatically by Diesel CLI.

diesel::table! {
    chessboard_square (id) {
        id -> Int4,
        game_id -> Int4,
        rank -> Int2,
        file -> Int2,
        max_sequence_index -> Int2,
        piece_type -> Nullable<Text>,
        piece_colour -> Nullable<Text>,
    }
}

diesel::table! {
    game (id) {
        id -> Int4,
        status -> Text,
    }
}

diesel::joinable!(chessboard_square -> game (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    chessboard_square,
    game,
);
