use std::hash::Hash;

use dashmap::DashSet;

use crate::utils::async_dash::AsyncDashSet;

pub struct DashSetLock<T>
where
    T: Eq + Hash + Clone,
{
    set: AsyncDashSet<T>,
}

impl<T> Default for DashSetLock<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DashSetLock<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            set: DashSet::new().into(),
        }
    }

    pub fn lock(&self, key: T) -> Option<DashSetLockGuard<T>> {
        DashSetLockGuard::new(self, key)
    }
}

pub struct DashSetLockGuard<'a, T>
where
    T: Eq + Hash + Clone,
{
    lock: &'a DashSetLock<T>,
    key: T,
}

impl<'a, T> DashSetLockGuard<'a, T>
where
    T: Eq + Hash + Clone,
{
    fn new(lock: &'a DashSetLock<T>, key: T) -> Option<Self> {
        let guard = Self { lock, key };
        if guard.lock() {
            // the key was already in the set
            Some(guard)
        } else {
            None
        }
    }

    fn lock(&self) -> bool {
        self.lock.set.insert(self.key.clone())
    }

    fn release(&self) {
        self.lock.set.remove(&self.key);
    }
}

impl<T> Drop for DashSetLockGuard<'_, T>
where
    T: Eq + Hash + Clone,
{
    fn drop(&mut self) {
        self.release()
    }
}
