use chapter15::cons::List2::{Cons as Cons2, Nil as Nil2};
use chapter15::mbox::MyBox;
use chapter15::pointer::CustomSmartPointer;
use chapter15::rc_pointer;
use chapter15::cell_pointer;

fn main() {
    // use chapter15::cons::List1::{Cons as Cons1, Nil as Nil1, Nil};
    // let _a = Cons1(1, (Cons1(2, (Cons1(3, (Nil1))))));

    let _b = Cons2(1, Box::new(Cons2(2, Box::new(Cons2(3, Box::new(Nil2))))));

    let c: MyBox<u32> = MyBox::new(123);

    let _d = MyBox::new(c);
    // *d;

    let e = CustomSmartPointer {
        data: String::from("my stuff")
    };
    println!("CustomSmartPointer created, e={}", e.data);

    let f = CustomSmartPointer {
        data: String::from("some data")
    };
    println!("CustomSmartPointer created, f={}", f.data);
    drop(f);

    // --------------
    rc_pointer::run();

    cell_pointer::run();
}
