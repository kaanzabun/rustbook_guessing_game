#[allow(unused)]
use std::{io::{self, stdin, Read}, string};
use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(0..=255);
    println!("Selected number is {}", secret_number);
    let mut total_tries:u32 = 0;
    
    loop {
        let mut guess = String::new();
        println!("Guess the number between 0 to 255");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u8 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("You need to input an integer!"); continue;}
        };
        total_tries = total_tries + 1;
        println!("Your guess was {}.", guess);

        match guess.cmp(&secret_number) {    
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {println!("You win in {} tries", total_tries); break;},
            Ordering::Less => println!("{}", "Too small!".red())
        };
    }
}
