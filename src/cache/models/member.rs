use std::collections::HashSet;

use twilight_model::{
    gateway::payload::incoming::MemberUpdate,
    guild::Member,
    id::{marker::RoleMarker, Id},
};

pub struct CachedMember {
    pub nickname: Option<String>,
    pub server_avatar_url: Option<String>,
    pub roles: HashSet<Id<RoleMarker>>,
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
            roles: HashSet::from_iter(member.roles),
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
            roles: HashSet::from_iter(member.roles.to_owned()),
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
            roles: HashSet::from_iter(member.roles.to_owned()),
        }
    }
}
