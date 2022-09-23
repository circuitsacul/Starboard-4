use twilight_model::user::User;

pub struct CachedUser {
    pub is_bot: bool,
    pub name: String,
    pub avatar_url: Option<String>,
}

impl From<&User> for CachedUser {
    fn from(user: &User) -> Self {
        Self {
            is_bot: user.bot,
            name: user.name.clone(),
            avatar_url: user
                .avatar
                .map(|av| format!("https://cdn.discordapp.com/avatars/{}/{}.png", user.id, av)),
        }
    }
}

impl From<User> for CachedUser {
    fn from(user: User) -> Self {
        Self {
            is_bot: user.bot,
            name: user.name,
            avatar_url: user
                .avatar
                .map(|av| format!("https://cdn.discordapp.com/avatars/{}/{}.png", user.id, av)),
        }
    }
}
