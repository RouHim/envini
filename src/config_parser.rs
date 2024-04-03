use std::env;

use ini::Ini;

/// Holds the mapping between the environment variable and the INI file
#[derive(Debug, Clone, Default)]
pub struct ConfigEntry {
    pub env_var_name: String,
    pub ini_file: String,
    pub ini_section: Option<String>,
    pub ini_property_name: String,
    pub ini_property_value: Option<String>,
    pub expand: bool,
}

/// Parses the app configuration
/// If nothing is profided we are looking for envini_mapping.ini in the same directory
/// If the file is not found we will panic
pub fn parse(app_args: Vec<String>) -> Vec<ConfigEntry> {
    if app_args.len() > 1 {
        parse_config(app_args[1].clone())
    } else if std::path::Path::new("envini_mapping.ini").exists() {
        parse_config("envini_mapping.ini".to_string())
    } else {
        panic!("No INI file provided and no envini_mapping.ini found in the current directory");
    }
}

/// Parses the configuration from the provided INI file
/// # Example ini file
/// ```ini
/// [KF2_WEB_ADMIN_PASSWORD]
/// ini_file = ~/test.ini
/// ini_section = Engine.AccessControl
/// ini_key = AdminPassword
/// ```
fn parse_config(ini_file_path: String) -> Vec<ConfigEntry> {
    let ini_data = Ini::load_from_file(ini_file_path).unwrap();
    ini_data
        .sections()
        .flatten()
        .map(|env_name_section| to_config_entry(&ini_data, env_name_section))
        .collect()
}

/// Converts a section of the INI file to a ConfigEntry
/// # Example ini entry
/// ```ini
/// [KF2_WEB_ADMIN_PASSWORD]
/// ini_file = ~/Downloads/test.ini
/// ini_section = Engine.AccessControl
/// ini_key = AdminPassword
/// ```
fn to_config_entry(ini_data: &Ini, env_config_section_name: &str) -> ConfigEntry {
    let ini_config_section = ini_data.section(Some(env_config_section_name)).unwrap();
    let env_name = env_config_section_name.to_string();
    let ini_property_value = env::var(&env_name).ok();

    let ini_file = ini_config_section
        .get("ini_file")
        .unwrap_or_else(|| panic!("No ini_file found for section {}", env_config_section_name))
        .to_string();

    let ini_section = ini_config_section
        .get("ini_section")
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty());

    let ini_property_name = ini_config_section
        .get("ini_key")
        .unwrap_or_else(|| panic!("No ini_key found for section {}", env_config_section_name))
        .to_string();

    let expand = ini_config_section
        .get("expand")
        .map(|s| s.parse::<bool>().unwrap())
        .unwrap_or(false);

    let _create = ini_config_section
        .get("create")
        .map(|s| s.parse::<bool>().unwrap())
        .unwrap_or(false);

    ConfigEntry {
        env_var_name: env_name,
        ini_file,
        ini_section,
        ini_property_name,
        ini_property_value,
        expand,
    }
}
