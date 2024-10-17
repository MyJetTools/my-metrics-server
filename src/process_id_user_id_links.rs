use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::*};

#[derive(Debug)]
pub struct ProcessIdUserIdLink {
    pub process_id: i64,
    pub user_id: String,
    pub created_at: DateTimeAsMicroseconds,
}

impl EntityWithKey<i64> for ProcessIdUserIdLink {
    fn get_key(&self) -> &i64 {
        &self.process_id
    }
}

pub struct ProcessIdUserIdLinks {
    pub items: SortedVec<i64, ProcessIdUserIdLink>,
}

impl ProcessIdUserIdLinks {
    pub fn new() -> Self {
        Self {
            items: SortedVec::new(),
        }
    }

    pub fn update(&mut self, process_id: i64, user_id: &str) {
        match self.items.insert_or_if_not_exists(&process_id) {
            InsertIfNotExists::Insert(insert_entity) => {
                insert_entity.insert(ProcessIdUserIdLink {
                    process_id,
                    user_id: user_id.to_string(),
                    created_at: DateTimeAsMicroseconds::now(),
                });
            }
            InsertIfNotExists::Exists(_) => {}
        }
    }

    pub fn gc(&mut self) {
        let now = DateTimeAsMicroseconds::now();
        let mut to_remove = Vec::new();
        for itm in self.items.iter() {
            if (now - itm.created_at).get_full_seconds() >= 20 {
                to_remove.push(itm.process_id);
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
