use std::env;
use std::path::PathBuf;

use ini::Ini;

pub const TEST_INI_FILE_GOOD_1: &str = r#"
[KF2_SERVER_NAME]
ini_file = test.ini
ini_section = Engine.GameReplicationInfo
ini_key = ServerName
"#;

pub const TEST_INI_FILE_NO_SECTION_1: &str = r#"
[KF2_ENCODING]
ini_file = test.ini
ini_section =
ini_key = Encoding
"#;

pub const TEST_INI_FILE_NO_SECTION_2: &str = r#"
[KF2_ENCODING]
ini_file = test.ini
ini_section = " "
ini_key = Encoding
"#;

pub const TEST_INI_FILE_GOOD_2: &str = r#"
[KF2_SERVER_NAME]
ini_file = test.ini
ini_section = Engine.GameReplicationInfo
ini_key = ServerName

[KF2_WEB_ADMIN_PASSWORD]
ini_file = test.ini
ini_section = Engine.AccessControl
ini_key = AdminPassword
"#;

pub const TEST_INI_FILE_BAD_1: &str = r#"
[KF2_SERVER_NAME]
ini_file = test.ini
ini_section = Engine.GameReplicationInfo
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
    std::fs::remove_dir_all(ini_file.0.parent().unwrap()).unwrap();
}
