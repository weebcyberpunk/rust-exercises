use std::io::{self, Write};

fn factorial(mut num: u64) -> u64 {

    let mut r = num;

    loop {
        num = num - 1;
        if num == 0 {
            break r;
        }
        r = r * (num);
    }
}

fn main() {

    loop {
        let mut var = String::new();
        print!("Enter number: ");
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin()
            .read_line(&mut var)
            .expect("Failed to receive input.");

        let num: u64 = match var.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        println!("Factorial: {}", factorial(num));
        break;
    }
}
