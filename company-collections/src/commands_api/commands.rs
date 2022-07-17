use std::collections::HashMap;

#[derive(Debug)]
pub struct TwoArgs {
    pub arg1: String,
    pub arg2: String,
}

#[derive(Debug)]
pub enum Command {
    Add(TwoArgs),
    List(Vec<String>),
    Null,
}

pub fn add(args: TwoArgs, database: &mut HashMap<String, Vec<String>>) {

    let dep = database.entry(args.arg2).or_insert(Vec::new());
    dep.push(args.arg1);
}

pub fn list(args: Vec<String>, database: &mut HashMap<String, Vec<String>>) {
    if args.len() > 0 {
        for arg in args {
            match database.get(&arg) {
                Some(list) => {
                    println!("{}:", arg);
                    for employee in list {
                        println!("    {}", employee);
                    }
                },
                None => println!("No such department: {}", arg),
            }
        }
    } else {
        for (department, list) in database {
            println!("{}:", department);
            for employee in list {
                println!("    {}", employee);
            }
        }
    }
}
