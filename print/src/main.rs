fn main() {
    println!("My name is {0}, {1} {0}", "Bond", "James");

    println!("My name is {surname}, {name} {surname}", 
             surname="Bond", 
             name="James"
             );

    let string = format!("{} of {:b} people knows binary, the other half doesn't", 1, 2);
    println!("{}", string);
}
