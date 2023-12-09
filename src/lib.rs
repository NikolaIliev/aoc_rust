use std::{fs, path::Path};

pub fn read_input(year: &str, day: &str) -> String {
    return fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("inputs/year{}_day{}.txt", year, day)),
    )
    .unwrap()
    .trim_end_matches("\n")
    .to_string();
}
