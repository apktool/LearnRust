pub mod closure {
    pub fn run() {
        fn add_one_v1(x: u32) -> u32 { x + 1 }
        println!("function: {}", add_one_v1(1));

        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        println!("annotated closure: {}", add_one_v2(1));

        let add_one_v3 = |x| { x + 1 };
        println!("closure: {}", add_one_v3(1));

        let add_one_v4 = |x| x + 1;
        println!("closure: {}", add_one_v4(1));

        let list1 = vec![1, 2, 3];
        let only_borrows = || println!("From closure: {:?}", list1);
        println!("Before calling closure: {:?}", list1);
        only_borrows();
        println!("After calling closure: {:?}", list1);

        let mut list2 = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list2);
        let mut borrows_mutably = || list2.push(7);
        borrows_mutably();
        println!("After calling closure: {:?}", list2);
    }
}

pub mod iterator {
    pub fn run() {
        let v1 = vec![1, 2, 3];

        for value in v1.iter() {
            print!("{value}");
        }
        println!();

        let v2 = vec![9, 7, 1];
        let mut v2_iter = v2.iter();

        loop {
            match v2_iter.next() {
                Some(x) => print!("{}", x),
                None => break
            }
        }
        println!();

        // consuming adaptors
        let v3 = vec![8, 3, 1];
        let v3_sum: u32 = v3.iter().sum();
        println!("{}", v3_sum);

        // iterator adaptors
        let v3 = vec![6, 4, 2];
        let v3_res: Vec<u32> = v3.iter().map(|x| x + 1).collect();
        println!("{:?}", v3_res);
    }
}