use domain::entities::config::{AuthenticationSettings, Config};

pub trait AppConfigProvider {
    fn get_all(&self) -> anyhow::Result<Config>;
    fn get_auth_settings(&self) -> anyhow::Result<AuthenticationSettings>;
}
