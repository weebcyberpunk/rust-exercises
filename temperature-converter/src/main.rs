use std::io;
use std::io::Write;

fn ctof(input: f64) -> f64 {

    input * 1.8 + 32.0
}

fn main() {

    loop {
        print!("Enter Celsius value: ");
        io::stdout()
            .flush()
            .expect("Cannot flush stdout.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot receive input");

        let input: f64 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("Fahrenheit: {}", ctof(input));
        break;
    }
}
