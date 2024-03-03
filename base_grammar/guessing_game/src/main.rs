use rand::Rng;
use std::cmp::Ordering;
use std::io; //trait
fn main() {
    println!("猜數!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("神秘數字是: {}", secret_number);

    println!("猜測一個數");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("無法讀取行");

    // shodow
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("你猜測的數是: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"), //arm
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
