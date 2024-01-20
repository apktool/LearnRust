use std::collections::HashMap;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(3);
    v1.push(5);
    for item in v1 {
        print!("{item} ")
    }
    println!();

    let v2: Vec<i32> = Vec::from([9, 7, 5, 1]);
    println!("{:?}", v2);

    let mut v3: Vec<i32> = vec![2, 4];
    v3.push(6);
    v3.push(7);
    let x1: i32 = v3[2];
    let x2: &i32 = &v3[2];
    let x3: Option<&i32> = v3.get(2);
    println!("{x1} {x2} {:?}", x3);

    let v4 = vec![2, 3, 4];
    for item in &v4 {
        print!("{item} ")
    }
    println!();

    for item in v4 {
        print!("{item} ")
    }
    println!();

    let mut s1: String = String::new();
    s1.push('R');
    s1.push('U');
    s1.push('S');
    s1.push('T');
    println!("{s1}");

    let mut s2 = String::from("Hello World");
    println!("{s2}");

    s2.push_str(", Ok");
    println!("{}", s2.to_string());
    // println!(s2);

    let s3 = String::from("Hello ");
    let s4 = String::from("RUST");
    let s5 = s3 + &s4;
    println!("{s5}");

    let s6 = String::from("i");
    let s7 = String::from("am");
    let s8 = String::from("rust");
    let s9 = format!("{s6} {s7} {s8}");
    println!("{s9}");

    for b in s9.bytes() {
        print!("{} ", b);
    }
    println!();

    let mut t1: HashMap<String, i32> = HashMap::new();
    t1.insert(String::from("Blue"), 10);
    t1.insert(String::from("Yellow"), 20);
    for (color, score) in &t1 {
        println!("color={color}, score={score}");
    }

    let t2: HashMap<&str, i32> = HashMap::from([
        ("Alice", 99),
        ("Bob", 90)
    ]);

    for (name, age) in t2 {
        println!("name={name}, age={age}");
    }
}

