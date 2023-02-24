// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
    }
}
