use std::path::PathBuf;

// pub fn get_resource_as_string(file: &str) -> String {
//     let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     d.push(format!("tests/resources/{}", file));
//     std::fs::read_to_string(d.as_path()).unwrap()
// }

pub fn get_resource_as_file(file: &str) -> std::fs::File {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("tests/resources/{}", file));
    std::fs::File::open(d).unwrap()
}
