#[derive(Debug)]
struct User {
    username: String,
    password: String,
    phone: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated functions
    fn circle(self: &Self) -> u32 {
        2 * self.width + 2 * self.height
    }
}

impl Rectangle {
    fn show(self: &Self) {
        println!("Hello World");
    }
}

// function
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    let t = (30, 50);
    println!("{}", area(t));

    let user1 = User {
        username: String::from("apktool"),
        password: String::from("github.com"),
        phone: 911,
        active: true,
    };
    println!("username={}, password={}, phone={}, activate={}", user1.username, user1.password, user1.phone, user1.active);
    println!("{:?}", user1);

    let user2 = User {
        active: false,
        ..user1
    };
    println!("{:?}", user2);

    let black = Color(0, 0, 0);
    println!("R={},G={}, B={}", black.0, black.1, black.2);

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect);

    println!("{}", rect.area());
    println!("{}", rect.circle());

    rect.show()
}
