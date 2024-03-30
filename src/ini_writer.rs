use std::path::PathBuf;

use itertools::Itertools;

use crate::config_parser::ConfigEntry;

/// Writes the values to the INI files
pub fn write_values(entries: Vec<ConfigEntry>) {
    // Group by ini file
    let entries_grouped = entries.iter().group_by(|entry| entry.ini_file.clone());

    // Write entries per ini file
    for (ini_file, entries_per_file) in entries_grouped.into_iter() {
        let path_to_ini = PathBuf::from(ini_file);
        let mut ini_file = ini::Ini::load_from_file(&path_to_ini).unwrap();

        // Iter over entries for the current ini file
        for entry in entries_per_file {
            let section_name = entry.ini_section.clone();
            let property_name = entry.ini_property_name.clone();
            let property_value = entry.ini_property_value.clone();

            // Only set the value if it is present
            if let Some(property_value) = property_value {
                ini_file
                    .with_section(section_name)
                    .set(property_name, property_value);
            }
        }

        // Flush the changes to the INI file
        ini_file.write_to_file(path_to_ini).unwrap();
    }
}
