use assertor::{assert_that, BooleanAssertion, EqualityAssertion};

use crate::test_fixtures::TEST_ENVINI_CONFIG_GOOD_1;
use crate::{config_parser, test_fixtures};

// #![feature(custom_test_frameworks)]
// #![test_runner(my_runner)]
//
// #[cfg(test)]
// fn my_runner(ts: &[& dyn Fn(i32) -> String]) {
//     println!("Custom Test Framework running {} tests: ", ts.len());
//     for t in ts {
//         println!("{}", t(0));
//     }
// }

#[test]
fn test_parsing_with_valid_file() {
    // GIVEN is a test ini file
    // AND arguments to the app
    let envini_config_ini = test_fixtures::get_ini_file(TEST_ENVINI_CONFIG_GOOD_1);
    let args = vec![
        "".to_string(),
        envini_config_ini.0.to_str().unwrap().to_string(),
    ];

    // WHEN parsing the ini file
    let env_ini_mapping = config_parser::parse(args);

    // THEN the mapping should contain one entry
    // AND the entry should match the test ini file
    assert_that!(env_ini_mapping.len()).is_equal_to(1);
    let env_ini_mapping_entry = env_ini_mapping.first();
    assert_that!(env_ini_mapping_entry.is_some()).is_true();
    let env_ini_mapping_entry = env_ini_mapping_entry.unwrap();
    assert_that!(env_ini_mapping_entry.env_var_name).is_equal_to("KF2_SERVER_NAME".to_string());
    assert_that!(env_ini_mapping_entry.ini_file).is_equal_to("test.ini".to_string());
    assert_that!(env_ini_mapping_entry.ini_section.as_ref().unwrap())
        .is_equal_to(&"Engine.GameReplicationInfo".to_string());
    assert_that!(env_ini_mapping_entry.ini_property_name).is_equal_to("ServerName".to_string());

    test_fixtures::cleanup(envini_config_ini);
}

#[test]
fn test_parsing_with_valid_file_no_section() {
    // GIVEN is a test ini file
    // AND arguments to the app
    let envini_config_ini =
        test_fixtures::get_ini_file(test_fixtures::TEST_ENVINI_CONFIG_NO_SECTION_1);
    let args = vec![
        "".to_string(),
        envini_config_ini.0.to_str().unwrap().to_string(),
    ];

    // WHEN parsing the ini file
    let env_ini_mapping = config_parser::parse(args);

    // THEN the mapping should contain one entry
    // AND the entry should match the test ini file
    assert_that!(env_ini_mapping.len()).is_equal_to(1);
    let env_ini_mapping_entry = env_ini_mapping.first();
    assert_that!(env_ini_mapping_entry.is_some()).is_true();
    let env_ini_mapping_entry = env_ini_mapping_entry.unwrap();
    assert_that!(env_ini_mapping_entry.env_var_name).is_equal_to("KF2_ENCODING".to_string());
    assert_that!(env_ini_mapping_entry.ini_file).is_equal_to("test.ini".to_string());
    assert_that!(env_ini_mapping_entry.ini_section.as_ref()).is_equal_to(None);
    assert_that!(env_ini_mapping_entry.ini_property_name).is_equal_to("Encoding".to_string());

    test_fixtures::cleanup(envini_config_ini);
}

#[test]
fn test_parsing_with_valid_file_no_section_variant_2() {
    // GIVEN is a test ini file
    // AND arguments to the app
    let envini_config_ini =
        test_fixtures::get_ini_file(test_fixtures::TEST_ENVINI_CONFIG_NO_SECTION_2);
    let args = vec![
        "".to_string(),
        envini_config_ini.0.to_str().unwrap().to_string(),
    ];

    // WHEN parsing the ini file
    let env_ini_mapping = config_parser::parse(args);

    // THEN the mapping should contain one entry
    // AND the entry should match the test ini file
    assert_that!(env_ini_mapping.len()).is_equal_to(1);
    let env_ini_mapping_entry = env_ini_mapping.first();
    assert_that!(env_ini_mapping_entry.is_some()).is_true();
    let env_ini_mapping_entry = env_ini_mapping_entry.unwrap();
    assert_that!(env_ini_mapping_entry.env_var_name).is_equal_to("KF2_ENCODING".to_string());
    assert_that!(env_ini_mapping_entry.ini_file).is_equal_to("test.ini".to_string());
    assert_that!(env_ini_mapping_entry.ini_section.as_ref()).is_equal_to(None);
    assert_that!(env_ini_mapping_entry.ini_property_name).is_equal_to("Encoding".to_string());

    test_fixtures::cleanup(envini_config_ini);
}

#[test]
fn test_parsing_with_valid_file_two_entries() {
    // GIVEN is a test ini file
    // AND arguments to the app
    let envini_config_ini = test_fixtures::get_ini_file(test_fixtures::TEST_ENVINI_CONFIG_GOOD_2);
    let args = vec![
        "".to_string(),
        envini_config_ini.0.to_str().unwrap().to_string(),
    ];

    // WHEN parsing the ini file
    let env_ini_mapping = config_parser::parse(args);

    // THEN the mapping should contain two entries
    // AND the entries should match the test ini file
    assert_that!(env_ini_mapping.len()).is_equal_to(2);
    let env_ini_mapping_entry = env_ini_mapping.first();
    assert_that!(env_ini_mapping_entry.is_some()).is_true();
    let env_ini_mapping_entry = env_ini_mapping_entry.unwrap();
    assert_that!(env_ini_mapping_entry.env_var_name).is_equal_to("KF2_SERVER_NAME".to_string());
    assert_that!(env_ini_mapping_entry.ini_property_name).is_equal_to("ServerName".to_string());

    let env_ini_mapping_entry = env_ini_mapping.get(1);
    assert_that!(env_ini_mapping_entry.is_some()).is_true();
    let env_ini_mapping_entry = env_ini_mapping_entry.unwrap();
    assert_that!(env_ini_mapping_entry.env_var_name)
        .is_equal_to("KF2_WEB_ADMIN_PASSWORD".to_string());
    assert_that!(env_ini_mapping_entry.ini_property_name).is_equal_to("AdminPassword".to_string());

    test_fixtures::cleanup(envini_config_ini);
}

#[test]
#[should_panic]
fn test_parsing_with_invalid_file() {
    // GIVEN is a test ini file
    // AND arguments to the app
    let envini_config_ini = test_fixtures::get_ini_file(test_fixtures::TEST_ENVINI_CONFIG_BAD_1);
    let args = vec![
        "".to_string(),
        envini_config_ini.0.to_str().unwrap().to_string(),
    ];

    // WHEN parsing the ini file
    config_parser::parse(args);

    // THEN the parsing should panic

    // TODO: cleanup
}
