fn main() {
    let mut x = 5;
    println!("x value: {}", x);

    x = 6;
    println!("x value: {}", x);

    const MAX_POINT: u32 = 1000_00;
    println!("MAX_POINT value: {}", MAX_POINT);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y value: {}", y);

    /*
     * shadowed 일 시 형변환 가능
     */
    let spaces = "  ";
    let spaces = spaces.len();

    /*
     * mut을 이용 시 형변환 불가
     */
    /* let mut spaces = "  ";
    spaces = spaces.len(); */

    println!("spaces value: {}", spaces);

    let _tupple: (i32, f64, char) = (500, 2.2, 'a');

    let five_hundread = _tupple.0;
    let tow_point_two = _tupple.1;
    let a_char = _tupple.2;

    //let text = _tupple.3;
}
