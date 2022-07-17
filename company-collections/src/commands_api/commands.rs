#[derive(Debug)]
pub enum Command {
    Add(Vec<String>),
    List(Vec<String>),
    Null,
}

pub fn add(cmd: Command) {
    println!("Add command with {:?}", cmd);
}

pub fn list(cmd: Command) {
    println!("List command with {:?}", cmd);
}
