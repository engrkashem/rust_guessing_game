use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess a Number !");

    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("Your secret number is {}", secret_num);
    let mut count = 0;
    loop {
        if count > 10 {
            println!("You Lose. Reached max 10 guess.");
            return;
        }
        println!("Please Guess a number and Enter it: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please Enter a Number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        count += 1;
        println!("You Guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Congrats! Right Guess");
                break;
            }
        }
    }
    println!("You guessed {count} times");
}
