pub struct LazyLock<'s, T> {
    mutex: &'s tokio::sync::Mutex<T>,
    guard: Option<tokio::sync::MutexGuard<'s, T>>,
}

impl<'s, T> LazyLock<'s, T> {
    pub fn new(mutex: &'s tokio::sync::Mutex<T>) -> Self {
        Self { mutex, guard: None }
    }

    /*
    pub async fn get(&mut self) -> &T {
        if self.guard.is_none() {
            self.guard = Some(self.mutex.lock().await);
        }

        self.guard.as_ref().unwrap()
    }
     */

    pub async fn get_mut(&mut self) -> &mut T {
        if self.guard.is_none() {
            self.guard = Some(self.mutex.lock().await);
        }

        self.guard.as_mut().unwrap()
    }
}
