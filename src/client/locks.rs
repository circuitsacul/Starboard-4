use crate::utils::dashset_lock::DashSetLock;

#[derive(Default)]
pub struct Locks {
    pub post_update_lock: DashSetLock<i64>,
}

impl Locks {
    pub fn new() -> Self {
        Self::default()
    }
}
