use std::io;

fn factorial(mut num: u64) -> u64 {

    let mut r = num;

    loop {
        if num == 0 {
            break r;
        }
        r = r * (num);
        num = num - 1;
    }
}

fn main() {

    loop {
        let mut var = String::new();
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
