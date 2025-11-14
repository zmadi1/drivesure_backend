use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::user::User;
use crate::utils::responses::ApiError;

#[derive(Clone)]
pub struct UserStore {
    inner: Arc<Mutex<HashMap<Uuid, User>>>,   // key = Uuid
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Insert user and return the cloned user on success.
    pub async fn create_user(&self, user: User) -> Result<User, ApiError> {
        let store = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let mut s = store.lock().map_err(|_| ApiError::Internal("lock poisoned".into()))?;
            if s.values().any(|u| u.email == user.email) {
                return Err(ApiError::Internal("User with this email already exists".into()));
            }
            let user_clone = user.clone();
            s.insert(user.id, user);
            Ok(user_clone)
        })
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, ApiError> {
        let store = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let s = store.lock().map_err(|_| ApiError::Internal("lock poisoned".into()))?;
            Ok(s.values().cloned().collect())
        })
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    }

    pub async fn find_by_email(&self, email: String) -> Result<Option<User>, ApiError> {
        let store = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let s = store.lock().map_err(|_| ApiError::Internal("lock poisoned".into()))?;
            Ok(s.values().find(|u| u.email == email).cloned())
        })
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    }
}