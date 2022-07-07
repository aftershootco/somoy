use somoy::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for path in std::env::args().skip(1) {
        let orig = DateTimeOriginal::from_file(&path)?.as_string();
        let creat_date = CreateDate::from_file(&path)?.as_string();
        println!("DateTimeOriginal {:?}", orig);
        println!("DateTimeOriginal {:?}", creat_date);
    }
    Ok(())
}
