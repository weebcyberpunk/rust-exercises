use std::io::{self, Write};

fn main() {

    loop {
        print!("Enter floating point precision: ");
        io::stdout().flush().expect("Failed to flush stdout.");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to receive input.");
    
        let num: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };
        println!("PI is {0:.1$}", 3.141592, num);
        break;
    }
}
