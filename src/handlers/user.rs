
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use bcrypt::{hash, DEFAULT_COST};

use crate::models::user::{User, NewUser, CreateUserRequest, UserResponse};
use crate::utils::{success_response, error_response, internal_error_response};
use crate::database::DbPool;
use crate::schema::users;

pub async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let conn = &mut pool.get().map_err(|_| {
        internal_error_response()
    }).unwrap();

    let existing_user = users::table
        .filter(users::email.eq(&user_data.email))
        .select(User::as_select())
        .first(conn)
        .optional();

    if let Ok(Some(_)) = existing_user {
        return error_response("User with this email already exists", "EMAIL_EXISTS");
    }

    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return error_response("Failed to hash password", "PASSWORD_HASH_ERROR"),
    };

    let new_user = NewUser {
        email: user_data.email.clone(),
        password_hash: hashed_password,
        role: user_data.role.clone(),
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        phone: user_data.phone.clone(),
    };

    match diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)
    {
        Ok(user) => {
            let user_response = UserResponse::from(user);
            success_response(user_response, "User created successfully")
        }
        Err(e) => {
            println!("Database insertion error: {}", e);
            internal_error_response()
        }
    }
}

pub async fn get_users(
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let conn = &mut pool.get().map_err(|_| {
        internal_error_response()
    }).unwrap();

    match users::table
        .select(User::as_select())
        .load(conn)
    {
        Ok(users_list) => {
            let user_responses: Vec<UserResponse> = users_list.into_iter().map(UserResponse::from).collect();
            success_response(user_responses, "Users retrieved successfully")
        }
        Err(e) => {
            println!("Database error: {}", e);
            internal_error_response()
        }
    }
}
