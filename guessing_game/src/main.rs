use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // match pattern to ignore not number parse error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
