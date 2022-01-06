fn main() {
    let a = c_to_f(-2.0);
    println!("{}", a);

    let b = f_to_c(28.4);
    println!("{}", b);

    let fib2 = fibonacci(2);
    println!("{}", fib2);
    let fib3 = fibonacci(3);
    println!("{}", fib3);
    let fib4 = fibonacci(4);
    println!("{}", fib4);
    let fib5 = fibonacci(5);
    println!("{}", fib5);
    let fib6 = fibonacci(6);
    println!("{}", fib6);
    let fib7 = fibonacci(7);
    println!("{}", fib7);
    let fib8 = fibonacci(8);
    println!("{}", fib8);
}

fn c_to_f (c:f64) -> f64 {
    c*1.8 + 32.0
}

fn f_to_c (f:f64) -> f64 {
    (f - 32.0) / 1.8
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}