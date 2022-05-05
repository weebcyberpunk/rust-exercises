use std::io;

fn factorial(mut num: u64) -> u64 {

    let mut r = num;

    loop {
        num = num - 1;
        if num == 0 {
            break;
        }
        r = r * (num);
    }

    r
}

fn main() {

    let mut var = String::new();
    let num: u64;

    loop {
        io::stdin()
            .read_line(&mut var)
            .expect("Failed to receive input.");

        num = match var.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        break;
    }

    println!("Factorial: {}", factorial(num));
}
