use std::env;
use std::path::PathBuf;

use ini::Ini;

use crate::config_parser::ConfigEntry;

pub const TEST_ENVINI_CONFIG_GOOD_1: &str = r#"
[KF2_SERVER_NAME]
ini_file = test.ini
ini_section = Engine.GameReplicationInfo
ini_key = ServerName
"#;

pub const TEST_ENVINI_CONFIG_EXPAND_ENV_VAR: &str = r#"
[KF2_WORKSHOP_ITEMS]
ini_file = test.ini
ini_section = OnlineSubsystemSteamworks.KFWorkshopSteamworks
ini_key = ServerSubscribedWorkshopItems
expand = true
"#;

pub const TEST_ENVINI_CONFIG_NO_SECTION_1: &str = r#"
[KF2_ENCODING]
ini_file = test.ini
ini_section =
ini_key = Encoding
"#;

pub const TEST_ENVINI_CONFIG_NO_SECTION_2: &str = r#"
[KF2_ENCODING]
ini_file = test.ini
ini_section = " "
ini_key = Encoding
"#;

pub const TEST_ENVINI_CONFIG_GOOD_2: &str = r#"
[KF2_SERVER_NAME]
ini_file = test.ini
ini_section = Engine.GameReplicationInfo
ini_key = ServerName

[KF2_WEB_ADMIN_PASSWORD]
ini_file = test.ini
ini_section = Engine.AccessControl
ini_key = AdminPassword
"#;

pub const TEST_ENVINI_CONFIG_BAD_1: &str = r#"
[KF2_SERVER_NAME]
ini_file = test.ini
ini_section = Engine.GameReplicationInfo
"#;

pub const TEST_INI_FILE_TO_EXPAND: &str = r#"
[OnlineSubsystemSteamworks.KFWorkshopSteamworks]
"#;

pub const TEST_INI_FILE_GOOD_1: &str = r#"
[Engine.GameReplicationInfo]
GamePassword=KF2Password
"#;

pub const TEST_INI_FILE_GOOD_2: &str = r#"
[Engine.ServerActors]
ServerActors=IpDrv.MasterServerUplink
ServerPlaylist=KFGame.KFGameType
"#;

pub const TEST_INI_FILE_NO_SECTION_1: &str = r#"
Encoding=UTF-8
"#;

pub const TEST_INI_FILE_EMPTY_1: &str = r#"
"#;

/// Create an ini file in systems for test purposes
/// Returns the absolute path to the created ini file
pub fn get_ini_file(test_data: &str) -> (PathBuf, Ini) {
    let test_dir = ensure_test_dir();

    // Create random file, write test ini content to it and return the path
    let test_ini_file = test_dir.join(uuid::Uuid::new_v4().to_string());
    std::fs::write(&test_ini_file, test_data.trim()).unwrap();

    // Parse ini file
    let ini_data = Ini::load_from_str(test_data).unwrap();

    // Return the path to the created ini file
    // And the parsed ini data
    (test_ini_file, ini_data)
}

/// Ensure that the test directory exists
/// Returns the absolute path to the test directory
fn ensure_test_dir() -> std::path::PathBuf {
    // Create a random test directory in the systems temp directory
    let system_temp_dir = env::temp_dir();
    let test_dir = system_temp_dir
        .join("envini_test_dir")
        .join(uuid::Uuid::new_v4().to_string());

    // Cleanup the test directory if it exists
    if test_dir.exists() {
        std::fs::remove_dir_all(&test_dir).unwrap();
    }

    // Create the test directory
    std::fs::create_dir_all(&test_dir).unwrap();

    // Return the path to the test directory
    test_dir
}

/// Cleanup the parent directory of the ini file
pub fn cleanup(ini_file: (PathBuf, Ini)) {
    let file_path = ini_file.0.parent().unwrap();
    if file_path.exists() {
        std::fs::remove_dir_all(file_path).expect("Could not remove test directory");
    }
}

/// Creates an envini config from the given ini file
pub fn create_config(
    ini_file: &(PathBuf, Ini),
    new_value: &str,
    section: &str,
    property_name: &str,
) -> ConfigEntry {
    let env_var_name = format!(
        "ENVINI_{}",
        uuid::Uuid::new_v4()
            .to_string()
            .to_uppercase()
            .replace('-', "_")
    );
    env::set_var(&env_var_name, new_value);

    let section = if section.is_empty() {
        None
    } else {
        Some(section.to_string())
    };

    ConfigEntry {
        env_var_name,
        ini_file: ini_file.0.to_str().unwrap().to_string(),
        ini_section: section,
        ini_property_name: property_name.to_string(),
        ini_property_value: Some(new_value.to_string()),
        expand: false,
    }
}
