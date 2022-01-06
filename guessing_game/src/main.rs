use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Check number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("정답: {}", secret_number);

    loop {
        println!("정답이라고 생각하는 숫자르 ㄹ입혀가세요.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("적다"),
            Ordering::Greater => println!("크다"),
            Ordering::Equal => {
                println!("같다");
                break;
            }
        }
    }
    
}
