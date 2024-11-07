// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (id) {
        id -> Int4,
        block_number -> Int8,
        timestamp -> Nullable<Timestamptz>,
    }
}
