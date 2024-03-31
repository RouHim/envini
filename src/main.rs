use std::env;

mod config_parser;
mod ini_writer;

#[cfg(test)]
mod config_parser_test;
#[cfg(test)]
mod ini_writer_test;
#[cfg(test)]
mod test_fixtures;

fn main() {
    let args = env::args().collect();
    let env_ini_mappings = config_parser::parse(args);
    ini_writer::write_values(env_ini_mappings);
}
