use std::io;

fn main()  {

    let mut txt = String::new();

    loop {
        let cont = match io::stdin().read_line(&mut txt) {
            Ok(0) => false,
            Ok(..) => true,
            Err(_) => continue,
        };

        if cont {
            print!("{}", txt);
        } else {
            println!("{}", txt);
            break;
        }
    }
}
