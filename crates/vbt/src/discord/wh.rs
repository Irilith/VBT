use std::env;

pub fn weekly_url() -> Result<String, env::VarError> {
    env::var("WEEKLY")
}

pub fn processed_url() -> Result<String, env::VarError> {
    env::var("PROCESSED")
}
