use std::io;
use std::io::Read;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("test.txt")?;
    let mut string = String::new();
    f.read_to_string(&mut string)?;
    print!("{string}");

    Ok(())
}
