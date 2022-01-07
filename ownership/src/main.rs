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
    
    let ref_test = String::from("reference");
    let len = calculate_length(&ref_test);

    println!("{}{}", ref_test, len);

    //가변 참조
    let mut mut_ref_test = String::from("adaf");
    change(&mut mut_ref_test);

    println!("{}", mut_ref_test);

    let mut ss = String::from("asdf asdf asdf asdf");
    let word = first_word(&ss);

    println!("{}", word);

    ss.clear();

    println!("{}", word);
}

//slice
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
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

fn calculate_length(ref_string: &String) -> usize {
    ref_string.len()
}

fn change(ref_string: &mut String) -> () {
    ref_string.push_str(", asdfadsf");
}
