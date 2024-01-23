pub mod cons {
    /*! recursive without indirection */
    /*
    pub enum List1 {
        Cons(i32, List1),
        Nil,
    }
    */

    pub enum List2 {
        Cons(i32, Box<List2>),
        Nil,
    }
}

pub mod mbox {
    use std::ops::Deref;

    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

pub mod pointer {
    pub struct CustomSmartPointer {
        pub data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
}

pub mod rc_pointer {
    use std::rc::Rc;

    struct Foo {
        data: u32,
    }

    pub fn run() {
        let foo = Rc::new(Foo { data: 42 });
        println!("Count<1>: {}", Rc::strong_count(&foo));
        let _foo1 = Rc::clone(&foo);
        println!("Count<2>: {}", Rc::strong_count(&foo));
        let _foo2 = Rc::clone(&foo);
        println!("Count<3>: {}", Rc::strong_count(&foo));
    }
}

pub mod cell_pointer {
    use std::cell::{Cell, RefCell};

    ///
    /// 想要通过 a2 修改 a1 的值（因为 a2 引用了 a1），一般是不可行的
    /// 而 Cell 以及 RefCell 就提供了可以修改的后门机制，可以通过 a2 修改 a1
    /// 这两个都是线程不安全的，无法用到多线程环境中
    pub fn run() {
        let a1 = 1;
        let a2 = &a1;
        // a2 = 2;
        println!("a1={:?}", a1);

        let b1 = Cell::new(1);
        let b2 = &b1;
        b2.set(2);
        println!("b1={:?}", b1);

        let c1 = RefCell::new(1);
        let c2 = &c1;
        *c2.borrow_mut() = 2;
        println!("c1={:?}", c2);
    }
}