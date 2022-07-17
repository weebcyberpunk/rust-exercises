use std::io::stdin;
use company_collections::def_commands;

fn main() {
    loop {
        let mut txt = String::new();
        match stdin().read_line(&mut txt) {
            Ok(0) => break,
            Ok(..) => def_commands::cmd_parse(&txt),
            Err(_) => break,
        }
    }
}
