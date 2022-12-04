use std::{fs, path::PathBuf};

pub struct TestCase<T> {
    pub input_path: &'static str,
    pub part1_expected: T,
    pub part2_expected: T,
}

impl<T> TestCase<T> {
    pub fn load_file(&self) -> String {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push(self.input_path);
        fs::read_to_string(input_path).unwrap()
    }
}
