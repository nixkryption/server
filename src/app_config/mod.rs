pub mod app_config {

    // Import all the required stuff
    use serde::{Deserialize, Serialize};

    // Derive for serde
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct AppConfig {
        pub debug: bool,
        pub fixversion: f64,
    }

    impl AppConfig {
        pub fn load() -> Result<Self, config::ConfigError> {
            // Load env from file via config
            let env = config::Config::builder()
                .add_source(config::File::with_name("env.toml"))
                .add_source(config::Environment::with_prefix("APP"))
                .build()?;

            env.try_deserialize()
        }
    }

    pub fn config() -> AppConfig {
        AppConfig::load().expect("Failed to load TOML configuration")
    }
}
