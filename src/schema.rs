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
    mandate_status (name) {
        name -> Text,
    }
}

diesel::table! {
    payment_mandates (id) {
        id -> Uuid,
        vehicle_id -> Uuid,
        owner_id -> Uuid,
        driver_id -> Uuid,
        amount_cents -> Int4,
        retry_count -> Int2,
        status -> mandate_status,
        due_date -> Date,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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

diesel::table! {
    vehicles (id) {
        id -> Uuid,
        owner_id -> Uuid,
        make -> Text,
        model -> Text,
        year -> Int4,
        reg_number -> Text,
        imei -> Nullable<Text>,
        can_immobilise -> Bool,
        is_available -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(driver_verifications -> users (user_id));
diesel::joinable!(payment_mandates -> users (owner_id));
diesel::joinable!(payment_mandates -> users (driver_id));
diesel::joinable!(payment_mandates -> vehicles (vehicle_id));

diesel::allow_tables_to_appear_in_same_query!(
    driver_verifications,
    mandate_status,
    payment_mandates,
    users,
    vehicles,
);
