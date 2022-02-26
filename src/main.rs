use rand::Rng; //Rng traits
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guessing game");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //expect-to handle the expected errors

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // println!("{}",err);
                println!("please enter a number");
                continue;
            },
        }; //let guess again? this is called shadowing

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
            Ordering::Greater => println!("too big"),
        }
    }
}