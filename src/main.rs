use somoy::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for path in std::env::args().skip(1) {
        let orig = DateTimeOriginal::from_file(&path)?;
        println!("DateTimeOriginal {:>22}", orig.as_string());
        let creat_date = CreateDate::from_file(&path)?;
        println!("CreateDate {:>28}", creat_date.as_string());
    }
    Ok(())
}
