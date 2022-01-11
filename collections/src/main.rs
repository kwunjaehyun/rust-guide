fn main() {
    let mut v = vec![1,2,3, 4, 5];
    let thrid: &i32 = &v[2];
    println!("세번째item: {}", thrid);

    match v.get(2) {
        Some(thrid) => println!("세번째원소: {}", thrid),
        None => println!("없음")
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i)
    }
    //let a = vec![1,2,"as"];

    let mut s1 = String::from("foo");
    let s2 = "bar";
    let s3 = String::from("bar2");
    s1.push_str(s2);
    s1.push_str(s3.as_str());
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s1);
}
