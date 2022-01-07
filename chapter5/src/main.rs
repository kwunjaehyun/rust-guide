fn main() {
    println!("Hello, world!");

    let user1 = User{
        name: String::from("재ㅎㄴ"),
        email: String::from("adf"),
        active: true
    };

    let user2 = build_user(String::from("asdb"), String::from("baef"));

    let rect = Rectangle {
        width: 8,
        height: 9
    };

    println!("rect1: {}", rect.area());

    let rect2 = Rectangle::square(4);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectagnle {width: size, height: size}
    }
}

struct User {
    name : String,
    email: String,
    active: bool
}

fn build_user(email: String, name: String) -> User {
    User {
        name,
        email,
        active: true
    }
}
