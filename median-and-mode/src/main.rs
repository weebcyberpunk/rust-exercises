//
// Receives a list of integers from stdin, prints the median and the mode.
//

use std::io::stdin;
use std::collections::HashMap;

fn main() {
    loop {
        let mut txt = String::new();

        let mut nums: Vec<isize> = match stdin().read_line(&mut txt) {
            Ok(0) => break,
            Ok(..) => {
                let mut new_vec: Vec<isize> = Vec::new();
                for word in txt.split_whitespace() {
                    match word.parse() {
                        Ok(num) => new_vec.push(num),
                        Err(_) => {
                            println!("Not a number: {}", word);
                            continue;
                        }
                    }
                }
                new_vec
            },
            Err(_) => break,
        };

        if nums.len() > 0 {
            nums.sort();
            median_and_mode(&mut nums);
        }
    }
}

fn median_and_mode(vector: &Vec<isize>) {

    let vec_size = vector.len();

    let mut sum = 0;
    for num in vector {
        sum = sum + *num;
    }
    print!("median: {} ", sum as f64 / vec_size as f64);
    print!("middle: {} ", vector[vec_size / 2]);

    let mut table = HashMap::new();
    for num in vector {
        let count = table.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode = (0, 0);
    for (num, count) in table {
        if count > mode.1 {
            mode.1 = count;
            mode.0 = *num;
        }
    }

    println!("mode: {} ({} times)", mode.0, mode.1);
}
