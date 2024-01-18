/**
 * ownership
 * Rust 中的每一个值都有一个 所有者（owner）。
 * 值在任一时刻有且只有一个所有者。
 * 当所有者（变量）离开作用域，这个值将被丢弃。
 * reference
 * 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
 * 引用必须总是有效的。
 * slice
 * 是一类引用，所以它没有所有权
 */
fn main() {
    let s1 = String::from("Hello World");

    let s2 = s1;
    println!("{}", s2);

    // ERROR: value borrowed here after move
    // println!("{}", s1);

    // ERROR: value borrowed here after move
    // let s3 = s1.clone();

    let s4 = s2.clone();
    println!("{}", s4);

    let x1 = String::from("Hello Rust");
    take_ownership(x1);
    // ERROR: value borrowed here after move
    // println!("{}", x1);

    let x2 = String::from("Hello Rust1");
    let x3: String = take_ownership_return1(x2);
    println!("{}", x3);

    let x4 = String::from("Hello Rust2");
    let x5: String = take_ownership_return2(x4);
    println!("{}", x5);

    let x6: String = give_ownership();
    println!("{}", x6);

    let y1: u32 = 5;
    make_copy(y1);
    println!("{}", y1);

    let z1: String = String::from("Hello");
    // reference
    let z2: usize = cal_length(&z1);
    println!("{}", z2);

    let mut z3: String = String::from("Hello");
    change(&mut z3);
    println!("{}", z3);

    let mut z4: String = String::from("Hello");
    let r1 = &mut z4;
    let r2 = &mut z4;
    // ERROR: cannot borrow `z4` as mutable more than once at a time
    // println!("{} {}", r1, r2);

    let slice: String = String::from("Hello World");
    let slice1 = &slice[3..slice.len()];
    for item in slice1.chars() {
        print!("{}", item);
    }
    println!();

    let slice2 = &slice[..];
    for item in slice2.chars() {
        print!("{}", item)
    }
    println!();
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_ownership_return1(s: String) -> String {
    s
}

fn take_ownership_return2(s: String) -> String {
    return s;
}

fn give_ownership() -> String {
    let s: String = String::from("Hello Rust");
    return s;
}

fn make_copy(v: u32) {
    println!("{}", v)
}

// reference -> borrowing
fn cal_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" World");
}