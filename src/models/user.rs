use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub is_verified: Option<bool>,      // CHANGED: Option<bool>
    pub created_at: Option<DateTime<Utc>>, // CHANGED: Option
    pub updated_at: Option<DateTime<Utc>>, // CHANGED: Option
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            role: user.role,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            is_verified: user.is_verified.unwrap_or(false), // Handle Option
            created_at: user.created_at.unwrap_or(Utc::now()), // Handle Option
        }
    }
}
