use std::io::{self, Write};

fn main() {

    let a = [1, 2, 3, 4, 5];
    
    print!("Please enter an array index: ");
    io::stdout().flush().expect("Failed to flush stdout.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index.trim().parse().expect("Please enter a number!");

    let element = a[index];

    println!("The value of the {}th element of the array is {}.", index, element);
}
