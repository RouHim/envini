use std::path::PathBuf;
use assertor::{assert_that, EqualityAssertion, StringAssertion};
use itertools::Itertools;

use crate::test_fixtures::{
    cleanup, TEST_INI_FILE_EMPTY_1, TEST_INI_FILE_GOOD_1, TEST_INI_FILE_GOOD_2,
    TEST_INI_FILE_NO_SECTION_1, TEST_INI_FILE_TO_EXPAND,
};
use crate::{ini_writer, test_fixtures};

#[test]
fn test_write_values() {
    // GIVEN is some ini file that should be modified
    // AND a valid config entry that points to this ini
    // AND an envini config describing the env ini mapping
    let ini_file_to_modify = test_fixtures::get_ini_file(TEST_INI_FILE_GOOD_1);
    let new_value_to_set = uuid::Uuid::new_v4().to_string();
    let envini_config = test_fixtures::create_config(
        &ini_file_to_modify,
        &new_value_to_set,
        "Engine.GameReplicationInfo",
        "ServerName",
    );

    // WHEN the config entry is written to the ini file
    ini_writer::write_values(vec![envini_config.clone()]);

    // THEN the ini file should contain the expected values
    let ini_data = ini::Ini::load_from_file(&ini_file_to_modify.0).unwrap();
    let section = ini_data.section(envini_config.ini_section).unwrap();
    assert_that!(section.get(&envini_config.ini_property_name).unwrap())
        .is_same_string_to(envini_config.ini_property_value.unwrap());

    cleanup(ini_file_to_modify);
}

#[test]
fn test_write_values_empty_ini_file() {
    // GIVEN is an empty ini file that should be filled with a new value
    // AND a valid config entry that points to this ini
    // AND an envini config describing the env ini mapping
    let ini_file_to_modify = test_fixtures::get_ini_file(TEST_INI_FILE_EMPTY_1);
    let new_value_to_set = uuid::Uuid::new_v4().to_string();
    let envini_config = test_fixtures::create_config(
        &ini_file_to_modify,
        &new_value_to_set,
        "Engine.GameReplicationInfo",
        "ServerName",
    );

    // WHEN the config entry is written to the ini file
    ini_writer::write_values(vec![envini_config.clone()]);

    // THEN the ini file should contain the expected values
    let ini_data = ini::Ini::load_from_file(&ini_file_to_modify.0).unwrap();
    let section = ini_data.section(envini_config.ini_section).unwrap();
    assert_that!(section.get(&envini_config.ini_property_name).unwrap())
        .is_same_string_to(envini_config.ini_property_value.unwrap());

    cleanup(ini_file_to_modify);
}

#[test]
fn test_write_values_existing_ini_non_existent_ini_property() {
    // GIVEN is an existing ini file that should be filled with a new ini value
    // AND a valid config entry that points to this ini
    // AND an envini config describing the env ini mapping
    let ini_file_to_modify = test_fixtures::get_ini_file(TEST_INI_FILE_GOOD_1);
    let new_value_to_set = uuid::Uuid::new_v4().to_string();
    let envini_config = test_fixtures::create_config(
        &ini_file_to_modify,
        &new_value_to_set,
        "Engine.GameReplicationInfo",
        "ServerPassword",
    );

    // WHEN the config entry is written to the ini file
    ini_writer::write_values(vec![envini_config.clone()]);

    // THEN the ini file should contain the expected values
    let ini_data = ini::Ini::load_from_file(&ini_file_to_modify.0).unwrap();
    let section = ini_data.section(envini_config.ini_section).unwrap();
    assert_that!(section.get(&envini_config.ini_property_name).unwrap())
        .is_same_string_to(envini_config.ini_property_value.unwrap());

    cleanup(ini_file_to_modify);
}

#[test]
fn test_write_values_expand() {
    // GIVEN is some ini file that should be modified
    // AND a config entry that points to this ini with variable expansion
    // AND an envini config describing the env ini mapping
    let ini_file_to_modify = test_fixtures::get_ini_file(TEST_INI_FILE_TO_EXPAND);
    let new_value_to_set = "123,456,789";
    let mut envini_config = test_fixtures::create_config(
        &ini_file_to_modify,
        new_value_to_set,
        "OnlineSubsystemSteamworks.KFWorkshopSteamworks",
        "ServerSubscribedWorkshopItems",
    );
    envini_config.ini_property_name = "ServerSubscribedWorkshopItems".to_string();
    envini_config.expand = true;

    // WHEN the config entry is written to the ini file
    ini_writer::write_values(vec![envini_config.clone()]);

    // THEN the ini file should contain the all values
    let ini_data = ini::Ini::load_from_file(&ini_file_to_modify.0).unwrap();
    let section = ini_data.section(envini_config.ini_section).unwrap();
    let all_entries = section
        .get_all(&envini_config.ini_property_name)
        .collect_vec();
    assert_that!(all_entries.len()).is_equal_to(3);
    assert_that!(all_entries[0]).is_same_string_to("123");
    assert_that!(all_entries[1]).is_same_string_to("456");
    assert_that!(all_entries[2]).is_same_string_to("789");

    cleanup(ini_file_to_modify);
}

#[test]
fn test_write_values_no_section() {
    // GIVEN is some ini file that should be modified
    // AND a valid config entry that points to this ini
    // AND an envini config describing the env ini mapping
    let ini_file_to_modify = test_fixtures::get_ini_file(TEST_INI_FILE_NO_SECTION_1);
    let new_value_to_set = uuid::Uuid::new_v4().to_string();
    let envini_config =
        test_fixtures::create_config(&ini_file_to_modify, &new_value_to_set, "", "Encoding");

    // WHEN the config entry is written to the ini file
    ini_writer::write_values(vec![envini_config.clone()]);

    // THEN the ini file should contain the expected values
    let ini_data = ini::Ini::load_from_file(&ini_file_to_modify.0).unwrap();
    assert_that!(ini_data
        .general_section()
        .get(&envini_config.ini_property_name)
        .unwrap())
    .is_same_string_to(envini_config.ini_property_value.unwrap());

    cleanup(ini_file_to_modify);
}

#[test]
fn test_write_values_bad_file() {
    // GIVEN is empty ini file
    // AND a valid config entry that points to this ini
    let ini_file_to_modify = test_fixtures::get_ini_file(TEST_INI_FILE_EMPTY_1);

    // WHEN the config entry is written to the ini file
    let new_value_to_set = uuid::Uuid::new_v4().to_string();
    let envini_config =
        test_fixtures::create_config(&ini_file_to_modify, &new_value_to_set, "", "");
    ini_writer::write_values(vec![envini_config.clone()]);

    // THEN the function should not panic

    cleanup(ini_file_to_modify);
}

#[test]
fn test_write_values_non_existent_file() {
    // GIVEN is a non-existent ini file
    // AND a valid config entry that points to this ini
    let ini_file_to_modify = (PathBuf::from("non_existent.ini"), ini::Ini::new());

    // WHEN the config entry is written to the ini file
    let new_value_to_set = uuid::Uuid::new_v4().to_string();
    let mut envini_config =
        test_fixtures::create_config(&ini_file_to_modify, &new_value_to_set, "abc", "abc");
    envini_config.ini_file = "non_existent.ini".to_string();
    ini_writer::write_values(vec![envini_config.clone()]);

    // THEN the function not should panic

    cleanup(ini_file_to_modify);
}

#[test]
fn test_write_values_two_different_ini_files() {
    // GIVEN are two ini files that should be modified
    // AND a valid config entry that points to these ini files
    let ini_file_to_modify_1 = test_fixtures::get_ini_file(TEST_INI_FILE_GOOD_1);
    let ini_file_to_modify_2 = test_fixtures::get_ini_file(TEST_INI_FILE_GOOD_2);
    let new_value_to_set_1 = uuid::Uuid::new_v4().to_string();
    let new_value_to_set_2 = uuid::Uuid::new_v4().to_string();
    let envini_config_1 = test_fixtures::create_config(
        &ini_file_to_modify_1,
        &new_value_to_set_1,
        "Engine.GameReplicationInfo",
        "ServerPassword",
    );
    let envini_config_2 = test_fixtures::create_config(
        &ini_file_to_modify_2,
        &new_value_to_set_2,
        "Engine.ServerActors",
        "ServerActors",
    );

    // WHEN the config entries are written to the ini files
    ini_writer::write_values(vec![envini_config_1.clone(), envini_config_2.clone()]);

    // THEN the ini files should contain the expected values
    let ini_data_1 = ini::Ini::load_from_file(&ini_file_to_modify_1.0).unwrap();
    let section_1 = ini_data_1.section(envini_config_1.ini_section).unwrap();
    assert_that!(section_1.get(&envini_config_1.ini_property_name).unwrap())
        .is_same_string_to(envini_config_1.ini_property_value.unwrap());

    let ini_data_2 = ini::Ini::load_from_file(&ini_file_to_modify_2.0).unwrap();
    let section_2 = ini_data_2.section(envini_config_2.ini_section).unwrap();
    assert_that!(section_2.get(&envini_config_2.ini_property_name).unwrap())
        .is_same_string_to(envini_config_2.ini_property_value.unwrap());

    cleanup(ini_file_to_modify_1);
    cleanup(ini_file_to_modify_2);
}
