fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    //소유권이 넘어감
    //println!("{}", s);

    let x = 5;
    makes_copy(x);

    //i32 타입은 복사를 수행하므로 유효
    println!("{}", x);

    let takes = gives_ownership();
    println!("{}", takes);

    let shut_up = String::from("asdfadsf");
    let back = takes_and_gives_back(shut_up);

    println!("{}", back);
}

fn takes_ownership (string : String) -> () {
    println!("{}", string);
}

fn makes_copy (integ: i32) -> () {
    println!("{}", integ);
}

fn gives_ownership() -> String {
    let some_string = String::from("bvafeasdfa");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}