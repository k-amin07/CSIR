use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut max_guesses = (100 as f32).log2().ceil() as i32; 
    while max_guesses > 0 {
        println!("Guesses left: {}", max_guesses);
        print!("Please input your guess: ");

        let _ = io::stdout().flush();
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Not a number");
        // instead of panicking, we will let user make another guess if the input was not a number
        let guess: u32 =
            match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Input was not a number");
                    continue;
                }
            };

        print!("You guessed: {} --> ",guess);
        _ = io::stdout().flush();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You Won");
                break;
            }
            Ordering::Greater => println!("Too Big")
        }
        println!("");
        max_guesses = max_guesses - 1;
    }
    if max_guesses == 0 {
        println!("You lost");
    }
}
