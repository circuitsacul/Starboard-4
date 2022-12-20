use twilight_model::guild::Permissions;

pub fn manage_channels() -> Permissions {
    Permissions::MANAGE_CHANNELS
}

pub fn manage_messages() -> Permissions {
    Permissions::MANAGE_MESSAGES
}
