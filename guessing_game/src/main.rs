extern crate rand;

use std::io;
use rand::Rng; // provide a random number generator
use std::cmp::Ordering;

fn main() {
    println!("Guess the num!");

    let secret = rand::thread_rng() // get a copy of the rng
        .gen_range(1, 100);
    println!("The secret num is: {}", secret);

    loop {
        let mut guess = String::new();
        println!("Input your guess:");
        // print!("Input your guess:");
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse() { // compile error
            Ok(num) => num,
            Err(_) => continue,
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
