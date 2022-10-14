use std::io;

use rand::Rng;

fn main() {
    println!("Hello, world!");

    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..190);

    // println!("The secret number is: {secret_number}");

 
    let mut guess = String::new();

    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number)  {
        std::cmp::Ordering::Less => println!("too small"),
        std::cmp::Ordering::Equal => print!("you win"),
        std::cmp::Ordering::Greater => println!("to0 big"),
    }
    // println!("You guessed: {guess}");   
}
