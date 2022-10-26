// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        login -> Varchar,
        email -> Varchar,
        password -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}
