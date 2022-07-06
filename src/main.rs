use somoy::*;
fn main() {
    for path in std::env::args().skip(1) {
        dbg!(DateTimeOriginal::from_file(&path).unwrap().as_string());
        dbg!(CreateDate::from_file(&path).unwrap().as_string());
    }
}
