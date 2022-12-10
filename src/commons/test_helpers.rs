use std::{fs, path::PathBuf};

pub struct TestCase<P1, P2> {
    pub input_path: &'static str,
    pub part1_expected: P1,
    pub part2_expected: P2,
}

impl<P1, P2> TestCase<P1, P2> {
    pub fn load_file(&self) -> String {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push(self.input_path);
        fs::read_to_string(input_path).unwrap()
    }
}
