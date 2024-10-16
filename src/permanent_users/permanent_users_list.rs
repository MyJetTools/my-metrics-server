use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermanentUserPersistModel {
    pub user: String,
    pub created: i64,
}

pub struct PermanentUsersList {
    pub users: HashMap<String, DateTimeAsMicroseconds>,
}

impl PermanentUsersList {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn has_as_permanent(&self, user_id: &str) -> bool {
        self.users.contains_key(user_id)
    }

    pub fn get_all(&self) -> Vec<PermanentUserPersistModel> {
        let mut result = Vec::new();
        for itm in self.users.iter() {
            result.push(PermanentUserPersistModel {
                user: itm.0.clone(),
                created: itm.1.unix_microseconds,
            });
        }

        result
    }

    pub fn add_permanent_user(&mut self, user_id: String) -> Vec<PermanentUserPersistModel> {
        self.users.insert(user_id, DateTimeAsMicroseconds::now());

        self.get_all()
    }

    pub fn remove_permanent_user(&mut self, user_id: &str) -> Vec<PermanentUserPersistModel> {
        self.users.remove(user_id);
        self.get_all()
    }
}
