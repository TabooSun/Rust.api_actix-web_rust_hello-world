use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub client_origin_url: String,
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Provide missing environment variables for Config")
    }
}
