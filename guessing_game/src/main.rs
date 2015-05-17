use std::io;

fn main() {
    println!("Guess the num!");

    let mut guess = String::new();
    println!("Input your guess:");
    // print!("Input your guess:");
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
