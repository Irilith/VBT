use std::env;

pub fn daily_url() -> Result<String, env::VarError> {
    env::var("DAILY")
}

pub fn processed_url() -> Result<String, env::VarError> {
    env::var("PROCESSED")
}
