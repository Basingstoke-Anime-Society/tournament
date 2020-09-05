table! {
    anime (id) {
        id -> Integer,
        name -> Varchar,
        episodes -> Nullable<Integer>,
        slot1 -> Bool,
        slot2 -> Bool,
        slot3 -> Bool,
    }
}

table! {
    decisions (tournament, left_anime, right_anime) {
        tournament -> Integer,
        left_anime -> Integer,
        right_anime -> Integer,
        pick -> Nullable<Bool>,
    }
}

table! {
    tournaments (id) {
        id -> Integer,
    }
}

table! {
    tournament_anime (tournament, anime) {
        tournament -> Integer,
        anime -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    anime,
    decisions,
    tournaments,
    tournament_anime,
);
