use actix_web::{web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;
use chrono::Utc;

use crate::models::{User, CreateUserRequest, UserResponse};
use crate::state::UserStore;
use crate::utils::{success_response, error_response};

pub async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    store: web::Data<UserStore>,
) -> HttpResponse {
    // Hash password
    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return error_response("Failed to hash password", "PASSWORD_HASH_ERROR"),
    };

    let user = User {
        id: Uuid::new_v4().to_string(),
        email: user_data.email.clone(),
        password_hash: hashed_password,
        role: user_data.role.clone(),
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        phone: user_data.phone.clone(),
        created_at: Utc::now(),
    };

    match store.create_user(user) {
        Ok(_) => {
            let user_response: UserResponse = store
                .find_by_email(&user_data.email)
                .ok()
                .flatten()
                .map(UserResponse::from)
                .expect("User should exist after creation");
            
            success_response(user_response, "User created successfully")
        }
        Err(e) => error_response(&e, "USER_CREATION_ERROR"),
    }
}

pub async fn get_users(store: web::Data<UserStore>) -> HttpResponse {
    match store.get_all_users() {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
            success_response(user_responses, "Users retrieved successfully")
        }
        Err(e) => error_response(&e, "FAILED_TO_RETRIEVE_USERS"),
    }
}
