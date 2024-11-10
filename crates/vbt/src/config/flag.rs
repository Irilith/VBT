use lazy_static::lazy_static;
use std::{env, sync::RwLock};

#[derive(Debug)]
pub struct EnvFlag {
    pub ft_webhook: bool,
}

lazy_static! {
    pub static ref FT_CONFIG: RwLock<EnvFlag> = RwLock::new(EnvFlag::load().unwrap_or_default());
}

impl EnvFlag {
    pub fn load() -> Result<Self, env::VarError> {
        let ft_webhook = env::var("FT_WEBHOOK")
            .map(|value| value.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);

        Ok(EnvFlag { ft_webhook })
    }
}

impl Default for EnvFlag {
    fn default() -> Self {
        EnvFlag { ft_webhook: false }
    }
}
