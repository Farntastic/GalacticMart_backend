// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Uuid,
        name -> Text,
        details -> Text,
        price -> Float8,
        stock -> Int4,
        image -> Text,
        category -> Varchar,
    }
}
