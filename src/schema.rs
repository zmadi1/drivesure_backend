// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        is_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}


// -- existing tables --
diesel::table! {
    vehicles (id) {
        id -> Uuid,
        owner_user_id -> Uuid,
        plate_number -> Varchar,
        make -> Varchar,
        model -> Varchar,
        year -> SmallInt,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
