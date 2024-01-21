pub mod traits {
    pub trait Summary {
        fn summary(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} {}", self.headline, self.author, self.location)
        }
    }
}

pub mod generics {
    use std::fmt::Display;

    #[derive(Debug)]
    pub struct Pair<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Pair<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        pub fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
// -------------------

pub mod lifetime {
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
