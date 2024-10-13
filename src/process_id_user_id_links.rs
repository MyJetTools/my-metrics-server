use rust_extensions::sorted_vec::*;

pub struct ProcessIdUserIdLink {
    pub process_id: i64,
    pub user_id: String,
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
                });
            }
            InsertIfNotExists::Exists(_) => {}
        }

        self.gc();
    }

    fn gc(&mut self) {
        while self.items.len() > MAX_ITEMS_AMOUNT {
            self.items.pop();
        }
    }

    pub fn resolve_user_id(&self, process_id: i64) -> Option<&str> {
        let result = self.items.get(&process_id)?;

        Some(&result.user_id)
    }
}
