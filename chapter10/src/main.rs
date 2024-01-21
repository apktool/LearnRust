use chapter10::generics::Pair;
use chapter10::traits::{NewsArticle, Summary};
use chapter10::lifetime;

fn main() {
    // trait
    let article = NewsArticle {
        headline: String::from("trait"),
        location: String::from("China"),
        author: String::from("Apktool"),
        content: String::from("This is article"),
    };
    let res = article.summary();
    println!("{}", res);

    // generics
    let p1 = Pair::new(1, 2);
    println!("{:?}", p1);
    p1.cmp_display();

    let p2 = Pair::new("c", "b");
    println!("{:?}", p2);
    p2.cmp_display();

    let p3 = lifetime::longest("abc", "y");
    println!("{}", p3);

    let s1: &'static str = "I have a static lifetime.";
    println!("{}", s1);
}
