use std::io;
use std::io::Write;

fn fibonacci(input: u64) {

    let mut a = 1;
    let mut b = 2;
    let mut c;
    let mut time = 0;

    while time < input {

        // were gonna print the last computed a and compute the next
        print!("{} ", a);
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        c = b;
        b = a + b;
        a = c;

        time += 1;
    }
}

fn main() {

    loop {

        let mut var = String::new();

        print!("Fibonacci: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");

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

        fibonacci(num);
        println!("");
        break;
    }
}
