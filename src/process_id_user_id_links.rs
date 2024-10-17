use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::*};

#[derive(Debug)]
pub struct ProcessIdUserIdLink {
    pub process_id: i64,
    pub user_id: String,
    pub created: DateTimeAsMicroseconds,
}

impl EntityWithKey<i64> for ProcessIdUserIdLink {
    fn get_key(&self) -> &i64 {
        &self.process_id
    }
}

const MAX_ITEMS_AMOUNT: usize = 1000;

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
                    created: DateTimeAsMicroseconds::now(),
                });
            }
            InsertIfNotExists::Exists(_) => {}
        }
    }

    pub fn gc(&mut self) {
        if self.items.len() <= MAX_ITEMS_AMOUNT {
            return;
        }

        let max_to_delete = self.items.len() - MAX_ITEMS_AMOUNT;

        let now = DateTimeAsMicroseconds::now();

        let mut to_delete = Vec::new();
        for itm in self.items.iter() {
            if (now - itm.created).get_full_seconds() >= 15 {
                to_delete.push(itm.process_id);

                if to_delete.len() >= max_to_delete {
                    break;
                }
            }
        }

        for itm in to_delete {
            self.items.remove(&itm);
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
