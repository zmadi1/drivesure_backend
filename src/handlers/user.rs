use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use crate::models::{User, NewUser, CreateUserRequest};
use crate::utils::{success_response, error_response, internal_error_response};
use crate::schema::users::dsl::*;
use bcrypt::{hash, DEFAULT_COST};

pub async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    pool: web::Data<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> HttpResponse {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    // Hash password
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

    match diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
    {
        Ok(user) => success_response(user, "User created successfully"),
        Err(e) => {
            if e.to_string().contains("duplicate key") {
                error_response("User with this email already exists", "EMAIL_EXISTS")
            } else {
                internal_error_response()
            }
        }
    }
}

pub async fn get_users(
    pool: web::Data<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> HttpResponse {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match users.load::<User>(&mut conn) {
        Ok(users_list) => success_response(users_list, "Users retrieved successfully"),
        Err(_) => internal_error_response(),
    }
}
