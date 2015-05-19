extern crate rand;

use std::io;
use rand::Rng; // provide a random number generator
use std::cmp::Ordering;

fn main() {
    let lower_bound = 1;
    let upper_bound = 101;
    println!("Guess the num! {} ~ {}", lower_bound, upper_bound);

    let secret = rand::thread_rng() // get a copy of the rng
        .gen_range(lower_bound, upper_bound);
    // println!("The secret num is: {}", secret);

    loop {
        let mut guess = String::new();
        println!("Input your guess:");
        // print!("Input your guess:");
        // io::stdout().flush();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a number!\n");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}
