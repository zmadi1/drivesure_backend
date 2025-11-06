use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::User;

#[derive(Clone)]
pub struct UserStore {
    inner: Arc<Mutex<HashMap<String, User>>>,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_user(&self, user: User) -> Result<(), String> {
        let mut store = self.inner.lock().map_err(|e| e.to_string())?;
        
        if store.values().any(|u| u.email == user.email) {
            return Err("User with this email already exists".to_string());
        }
        
        store.insert(user.id.clone(), user);
        Ok(())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, String> {
        let store = self.inner.lock().map_err(|e| e.to_string())?;
        Ok(store.values().cloned().collect())
    }

    pub fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let store = self.inner.lock().map_err(|e| e.to_string())?;
        Ok(store.values().find(|u| u.email == email).cloned())
    }
}
