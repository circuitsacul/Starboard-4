use crate::database::{
    helpers::settings::overrides::call_with_override_settings, Starboard, StarboardOverride,
    StarboardSettings,
};

#[derive(Debug)]
pub struct StarboardConfig {
    pub starboard: Starboard,
    pub overrides: Vec<StarboardOverride>,
    pub resolved: StarboardSettings,
}

macro_rules! update_from_override {
    ($settings: expr, $override: expr, $($field: ident),*) => {
        {
            $(
                match $override.$field {
                    Option::None => {},
                    Option::Some(value) => $settings.$field = value,
                }
            )*
        }
    };
}

impl StarboardConfig {
    pub fn new(
        starboard: Starboard,
        overrides: Vec<StarboardOverride>,
    ) -> serde_json::Result<Self> {
        let mut settings = starboard.settings.clone();
        for ov in overrides.iter() {
            call_with_override_settings!(update_from_override, settings, ov.get_overrides()?)
        }

        Ok(Self {
            starboard,
            overrides,
            resolved: settings,
        })
    }
}
