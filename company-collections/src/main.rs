use std::io::stdin;
use company_collections::commands_api;

fn main() {
    loop {
        let mut txt = String::new();
        let res = match stdin().read_line(&mut txt) {
            Ok(0) => break,
            Ok(..) => {
                commands_api::cmd_parse(&txt)
            },
            Err(_) => break,
        };

        match res {
            Ok(cmd) => commands_api::run_cmd(cmd),
            Err(_) => continue,
        }
    }
}
