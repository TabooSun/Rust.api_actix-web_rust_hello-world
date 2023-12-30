use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Auth0Config {
    pub audience: String,
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Default for Auth0Config {
    fn default() -> Self {
        envy::prefixed("AUTH0_")
            .from_env()
            .expect("Provide missing environment variables for Auth0Client")
    }
}

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
