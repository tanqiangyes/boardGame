// @generated automatically by Diesel CLI.

diesel::table! {
    user (id, name) {
        id -> Char,
        name -> Varchar,
        age -> Nullable<Integer>,
        password -> Char,
    }
}
