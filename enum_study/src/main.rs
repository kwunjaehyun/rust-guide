fn main() {
    /* let home = IpAddr {
        kind : IpAddrKind::V4,
        address: String::from("127")
    }; */

    let home = IpAddr::V4(String::from("asdf"));
    
    value_in_cents(Coin::Quater(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

#[derive(Debug)]
enum UsState {
    Alabama, Alaska
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quater(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater form {:?}!", state);
            25
        },
    }
}

enum IpAddr {
    V4(String),
    V6(String)
}

/* struct IpAddr {
    kind: IpAddrKind,
    address : String,
} */




