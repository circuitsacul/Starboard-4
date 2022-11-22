use twilight_model::{gateway::payload::incoming::MemberUpdate, guild::Member};

pub struct CachedMember {
    pub nickname: Option<String>,
    pub server_avatar_url: Option<String>,
}

impl From<Member> for CachedMember {
    fn from(member: Member) -> Self {
        Self {
            nickname: member.nick,
            server_avatar_url: member.avatar.map(|av| {
                format!(
                    "https://cdn.discordapp.com/avatars/{}/{}.png",
                    member.user.id, av
                )
            }),
        }
    }
}

impl From<&Member> for CachedMember {
    fn from(member: &Member) -> Self {
        Self {
            nickname: member.nick.as_ref().cloned(),
            server_avatar_url: member.avatar.as_ref().map(|av| {
                format!(
                    "https://cdn.discordapp.com/avatars/{}/{}.png",
                    member.user.id, av
                )
            }),
        }
    }
}

impl From<&MemberUpdate> for CachedMember {
    fn from(member: &MemberUpdate) -> Self {
        Self {
            nickname: member.nick.as_ref().cloned(),
            server_avatar_url: member.avatar.as_ref().map(|av| {
                format!(
                    "https://cdn.discordapp.com/avatars/{}/{}.png",
                    member.user.id, av
                )
            }),
        }
    }
}
