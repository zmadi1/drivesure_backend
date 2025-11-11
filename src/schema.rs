// @generated automatically by Diesel CLI.

diesel::table! {
    driver_verifications (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 30]
        licence_number -> Varchar,
        licence_front_url -> Text,
        selfie_url -> Text,
        face_match_score -> Nullable<Float4>,
        rtmc_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        is_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    vehicles (id) {
        id -> Int4,
        make -> Varchar,
        model -> Varchar,
        year -> Int4,
    }
}



diesel::joinable!(driver_verifications -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(driver_verifications, users,);
