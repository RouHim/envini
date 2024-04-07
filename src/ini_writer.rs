use std::path::PathBuf;

use itertools::Itertools;

use crate::config_parser::ConfigEntry;

/// Writes the values to the INI files
/// # Parameters
/// - `config_entries` - The configuration entries to write
pub fn write_values(config_entries: Vec<ConfigEntry>) {
    // Group by ini file
    let entries_grouped = config_entries
        .iter()
        .group_by(|entry| entry.ini_file.clone());

    // Write entries per ini file
    for (ini_file, config_entries) in entries_grouped.into_iter() {
        let path_to_ini = PathBuf::from(ini_file);
        let ini_file = ini::Ini::load_from_file(&path_to_ini);

        // If the file does not exist, skip it
        if ini_file.is_err() {
            println!("Could not load ini file: {:?}", path_to_ini);
            continue;
        }
        let mut ini_file = ini_file.unwrap();

        // Iter over entries for the current ini file
        for config_entry in config_entries {
            let section = config_entry.ini_section.clone();
            let property_name = config_entry.ini_property_name.clone();
            let property_value = config_entry.ini_property_value.clone();

            // Only set the value if it was set in the env var
            if let Some(property_value) = property_value {
                if config_entry.expand {
                    for split_property_value in property_value.split(',') {
                        ini_file
                            .with_section(section.clone())
                            .add(property_name.clone(), split_property_value);
                    }
                } else {
                    ini_file
                        .with_section(section)
                        .set(property_name, property_value);
                }
            }
        }

        // Flush the changes to the INI file
        ini_file.write_to_file(path_to_ini).unwrap();
    }
}
