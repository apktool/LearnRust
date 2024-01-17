fn main() {
    let mut x = 5;
    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x is {x}");
    }

    x = 6;
    println!("The value of x is {x}");


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The seconds is {THREE_HOURS_IN_SECONDS}s");

    /*
     * default: i32
     * 长度      有符号  无符号
     * 8-bit    i8     u8
     * 16-bit   i16    u16
     * 32-bit   i32    u32
     * 64-bit   i64    u64
     * 128-bit  i128   u128
     * arch     isize  usize
     */
    let a: u32 = "42".parse().expect("Not a number");
    println!("The value of a is {a}");

    /*
     * default: f64
     * 长度      符号
     * 32-bit   f32
     * 64-bit   f64
     */
    let b: f64 = 95.5 - 4.3;
    println!("The value of b is {b}");

    /*
     * bool
     */
    let c: bool = true;
    println!("The value of c is {c}");

    /*
     * char
     */
    let d: char = '😻';
    println!("The value of d is {d}");

    /*
     * tuple
     */
    let e: (i32, f64, u8) = (500, 3.14, 1);
    // destructuring
    let (e1, e2, e3) = e;
    println!("The value of e is ({e1}, {e2}, {e3})");

    /*
     * array
     */
    let f: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of f is [{}, {}]", f[0], f[1]);

    for elem in f {
        print!(" {elem}")
    }
    println!();

    for elem in f.iter().rev() {
        print!(" {elem}")
    }
    println!();

    let g: [i32; 5] = [3; 5];
    println!("The value of g is [{}, {}]", g[0], g[4]);

    let mut idx = 0;
    while idx < 5 {
        print!("{} ", g[idx]);
        idx += 1;
    }
    println!()
}
