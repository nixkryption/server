#[cfg(test)]
mod app_config_tests {
    use crate::app_config::app_config::AppConfig;
    use std::env;
    use std::fs;

    #[test]
    fn test_app_config_struct_creation() {
        let config = AppConfig {
            debug: true,
            fixversion: 4.4,
        };

        assert_eq!(config.debug, true);
        assert_eq!(config.fixversion, 4.4);
    }

    #[test]
    fn test_app_config_serialization() {
        let config = AppConfig {
            debug: false,
            fixversion: 1.0,
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.debug, deserialized.debug);
        assert_eq!(config.fixversion, deserialized.fixversion);
    }

    #[test]
    fn test_app_config_clone() {
        let original = AppConfig {
            debug: true,
            fixversion: 2.5,
        };

        let cloned = original.clone();

        assert_eq!(original.debug, cloned.debug);
        assert_eq!(original.fixversion, cloned.fixversion);
    }

    #[test]
    fn test_app_config_debug_trait() {
        let config = AppConfig {
            debug: true,
            fixversion: 3.14,
        };

        let debug_string = format!("{:?}", config);
        assert!(debug_string.contains("debug: true"));
        assert!(debug_string.contains("fixversion: 3.14"));
    }

    #[test]
    fn test_app_config_load_from_valid_toml() {
        let test_toml_content = r#"
debug = false
fixversion = 1.5
"#;

        let test_file_path = "test_env.toml";
        fs::write(test_file_path, test_toml_content).unwrap();

        let result = config::Config::builder()
            .add_source(config::File::with_name("test_env"))
            .build()
            .and_then(|env| env.try_deserialize::<AppConfig>());

        fs::remove_file(test_file_path).ok();

        match result {
            Ok(config) => {
                assert_eq!(config.debug, false);
                assert_eq!(config.fixversion, 1.5);
            }
            Err(_) => panic!("Should have successfully loaded config"),
        }
    }

    #[test]
    fn test_app_config_load_invalid_toml() {
        let test_toml_content = r#"
debug = "not_a_boolean"
fixversion = "not_a_number"
"#;

        let test_file_path = "test_invalid_env.toml";
        fs::write(test_file_path, test_toml_content).unwrap();

        let result = config::Config::builder()
            .add_source(config::File::with_name("test_invalid_env"))
            .build()
            .and_then(|env| env.try_deserialize::<AppConfig>());

        fs::remove_file(test_file_path).ok();

        assert!(result.is_err());
    }

    #[test]
    fn test_app_config_environment_override() {
        let test_toml_content = r#"
debug = false
fixversion = 1.0
"#;

        let test_file_path = "test_env_override.toml";
        fs::write(test_file_path, test_toml_content).unwrap();

        unsafe {
            env::set_var("APP_DEBUG", "true");
            env::set_var("APP_FIXVERSION", "5.5");
        }

        let result = config::Config::builder()
            .add_source(config::File::with_name("test_env_override"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .and_then(|env| env.try_deserialize::<AppConfig>());

        unsafe {
            env::remove_var("APP_DEBUG");
            env::remove_var("APP_FIXVERSION");
        }
        fs::remove_file(test_file_path).ok();

        match result {
            Ok(config) => {
                assert_eq!(config.debug, true);
                assert_eq!(config.fixversion, 5.5);
            }
            Err(_) => panic!("Should have successfully loaded config with env overrides"),
        }
    }

    #[test]
    fn test_app_config_missing_file() {
        let result = config::Config::builder()
            .add_source(config::File::with_name("nonexistent_file"))
            .build()
            .and_then(|env| env.try_deserialize::<AppConfig>());

        assert!(result.is_err());
    }

    #[test]
    fn test_app_config_partial_config() {
        let test_toml_content = r#"
debug = true
"#;

        let test_file_path = "test_partial_env.toml";
        fs::write(test_file_path, test_toml_content).unwrap();

        let result = config::Config::builder()
            .add_source(config::File::with_name("test_partial_env"))
            .build()
            .and_then(|env| env.try_deserialize::<AppConfig>());

        fs::remove_file(test_file_path).ok();

        assert!(result.is_err());
    }

    #[test]
    fn test_app_config_empty_file() {
        let test_file_path = "test_empty_env.toml";
        fs::write(test_file_path, "").unwrap();

        let result = config::Config::builder()
            .add_source(config::File::with_name("test_empty_env"))
            .build()
            .and_then(|env| env.try_deserialize::<AppConfig>());

        fs::remove_file(test_file_path).ok();

        assert!(result.is_err());
    }
}
