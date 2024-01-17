#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    print!("Hello print!");
    println!("Hello println!");
    eprint!("Error eprint!");
    eprintln!("Error eprintln!");

    println!("{} days", 31);
    println!("{}, this is {}", "Bob", "Alice");
    println!("{1}, this is {0}", "Alice", "Bob");
    println!("{key}, this is {value}", key = "Alice", value = "Bob");

    let a = format!("{}, this is {}", "Bob", "Alice");
    println!("{}", a);

    let b = format!("{} -> {a}", "ok");
    println!("{}", b);

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter)
}