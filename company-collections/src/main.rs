use std::io::stdin;
use std::io::{stdout, Write};
use std::collections::HashMap;
use company_collections::commands_api;

fn main() {

    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("maisíquel> ");
        stdout().flush().expect("Something really bad just happened");

        let mut txt = String::new();
        let res = match stdin().read_line(&mut txt) {
            Ok(0) => break,
            Ok(..) => {
                commands_api::cmd_parse(&txt)
            },
            Err(_) => break,
        };

        match res {
            Ok(cmd) => commands_api::run_cmd(cmd, &mut database),
            Err(_) => continue,
        }
    }
}
