use twilight_model::user::User;

pub struct CachedUser {
    pub is_bot: bool,
}

impl From<&User> for CachedUser {
    fn from(user: &User) -> Self {
        Self { is_bot: user.bot }
    }
}
