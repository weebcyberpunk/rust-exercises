use std::str::SplitWhitespace;
mod commands;
use crate::commands_api::commands::Command;

pub fn cmd_parse(txt: &String) -> Result<Command, ()> {

    let mut txt_it = txt.split_whitespace();
    let mut command = Command::Null;

    let ok = match txt_it.next() {
        Some(word) => { 
            if word == "add" {
                command = Command::Add(Vec::new());
            } else if word == "list" {
                command = Command::List(Vec::new());
            }

            known_cmd(&mut command, &mut txt_it)
        },
        None => {
            println!("No command!");
            false
        },
    };

    if ok {
        Ok(command)
    } else {
        Err(())
    }
}

pub fn run_cmd(cmd: Command) {

}

fn known_cmd(cmd: &mut Command, iter: &mut SplitWhitespace) -> bool {
    match cmd {
        Command::Add(ref mut args) => {
            args.push(String::new());
            let mut is_first = true;
            loop {
                match iter.next() {
                    Some("to") => {
                        if is_first {
                            println!("ERROR: Expected arguments before 'to'");
                            return false;
                        }
                        break;
                    },
                    Some(word) => {
                        if is_first {
                            is_first = false;
                        } else {
                            args[0].push(' ');
                        }
                        args[0].push_str(word);
                    },
                    None => {
                        if is_first {
                            println!("ERROR: Expected arguments after 'add'");
                        } else {
                            println!("ERROR: Expected 'to'");
                        }
                        return false;
                    }
                };
            }

            args.push(String::new());
            is_first = true;
            loop {
                match iter.next() {
                    Some(word) => {
                        if is_first {
                            is_first = false;
                        } else {
                            args[1].push(' ');
                        }
                        args[1].push_str(word);
                    },
                    None => {
                        if is_first {
                            println!("ERROR: Expected arguments after 'to'");
                            return false;
                        }
                        break;
                    },
                }
            }
        },
        Command::List(_) => println!("List command"),
        Command::Null => {
            println!("ERROR: unknown command.");
            return false;
        },
    };
    true
}
