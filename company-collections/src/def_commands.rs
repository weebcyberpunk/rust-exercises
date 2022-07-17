use std::str::SplitWhitespace;

enum Command {
    Add(Vec<String>),
    List(Vec<String>),
    Null,
}

pub fn cmd_parse(txt: &String) {
    let mut txt_it = txt.split_whitespace();

    match txt_it.next() {
        Some(word) => { 
            let mut command = Command::Null;
            if word == "add" {
                command = Command::Add(Vec::new());
            } else if word == "list" {
                command = Command::List(Vec::new());
            }

            known_cmd(command, txt_it);
        },
        None => println!("No command!"),
    }
}

fn known_cmd(cmd: Command, iter: SplitWhitespace) {
    match cmd {
        Command::Add(_) => println!("Add command"),
        Command::List(_) => println!("List command"),
        Command::Null => println!("Null command"),
    }
}
