use std::str::SplitWhitespace;
use std::collections::HashMap;
mod commands;
use crate::commands_api::commands::Command;
use crate::commands_api::commands::TwoArgs;

pub fn cmd_parse(txt: &String) -> Result<Command, ()> {

    let mut txt_it = txt.split_whitespace();
    let mut command = Command::Null;

    let ok = match txt_it.next() {
        Some(word) => { 
            if word == "add" {
                command = Command::Add(TwoArgs {arg1: String::new(), arg2: String::new()});
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

pub fn run_cmd(cmd: Command, database: &mut HashMap<String, Vec<String>>) {
    match cmd {
        Command::Add(args) => commands::add(args, database),
        Command::List(args) => commands::list(args, database),
        Command::Null => println!("Tried to run Null!"),
    }
}

fn known_cmd(cmd: &mut Command, iter: &mut SplitWhitespace) -> bool {
    match cmd {
        Command::Add(ref mut args) => {
            args.arg1 = String::new();
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
                            args.arg1.push(' ');
                        }
                        args.arg1.push_str(word);
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

            args.arg2 = String::new();
            is_first = true;
            loop {
                match iter.next() {
                    Some(word) => {
                        if is_first {
                            is_first = false;
                        } else {
                            args.arg2.push(' ');
                        }
                        args.arg2.push_str(word);
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
        Command::List(args) => {
            let mut buffer = String::new();
            let mut is_first = true;
            loop {
                match iter.next() {
                    Some("and") => {
                        if is_first {
                            println!("ERROR: Expected argument before 'and'!");
                            return false;
                        } else {
                            is_first = true;
                        }
                        args.push(buffer);
                        buffer = String::new();
                    },
                    Some(word) => {
                        if is_first {
                            is_first = false;
                        } else {
                            buffer.push(' ');
                        }
                        buffer.push_str(word)
                    },
                    None => {
                        if !is_first {
                            args.push(buffer);
                        }
                        break;
                    },
                }
            }
        },
        Command::Null => {
            println!("ERROR: unknown command.");
            return false;
        },
    };
    true
}
