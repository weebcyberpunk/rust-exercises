use std::io;

fn main() {
    loop {
        let mut txt = String::new();
        match io::stdin().read_line(&mut txt) {
            Ok(0) => break,
            Ok(..) => print!("{}", txt),
            Err(_) => continue,
        }
    }
}
