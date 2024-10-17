use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug)]
pub struct ProcessIdUserIdLink {
    pub user_id: String,
    pub created_at: DateTimeAsMicroseconds,
}

pub struct ProcessIdUserIdLinks {
    pub items: HashMap<i64, ProcessIdUserIdLink>,
}

impl ProcessIdUserIdLinks {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn update(&mut self, process_id: i64, user_id: &str) {
        if !self.items.contains_key(&process_id) {
            self.items.insert(
                process_id,
                ProcessIdUserIdLink {
                    user_id: user_id.to_string(),
                    created_at: DateTimeAsMicroseconds::now(),
                },
            );
        }
    }

    pub fn gc(&mut self) {
        let now = DateTimeAsMicroseconds::now();
        let mut to_remove = Vec::new();
        for (process_id, itm) in self.items.iter() {
            if (now - itm.created_at).get_full_seconds() >= 20 {
                to_remove.push(*process_id);
            }
        }

        for remove_key in to_remove {
            self.items.remove(&remove_key);
        }
    }

    pub fn resolve_user_id(&self, process_id: i64) -> Option<&str> {
        let result = self.items.get(&process_id)?;

        Some(&result.user_id)
    }

    pub fn get_size(&self) -> usize {
        self.items.len()
    }
}
