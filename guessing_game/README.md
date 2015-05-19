Guess the number
===
if I use `println!("Input your guess:");`, then it print out:

    >>> cargo run
       Compiling guessing_game v0.1.0 (file:///Users/JaySon/Projects/RustProjects/guessing_game)
         Running `target/debug/guessing_game`
    Guess the num!
    Input your guess:
    123
    You guessed: 123

but when I use `print!("Input your guess:");`, which I don't want to print `'\n'`, then it run to this case:

    >>> cargo run
       Compiling guessing_game v0.1.0 (file:///Users/JaySon/Projects/RustProjects/guessing_game)
         Running `target/debug/guessing_game`
    Guess the num!
    asdf
    Input your guess:You guessed: asdf

which doesn't flush the output buffer?
### About flush
Reference:[std::print! - Rust](http://doc.rust-lang.org/stable/std/macro.print!.html)  
> Note that stdout is frequently line-buffered by default so it may be necessary to use io::stdout().flush() to ensure the output is emitted immediately.

If you want to ensure the string output in order, you should use `io::stdout().flush()` to flush the `stdout`

* * *

This part make it error in compile stage:

        let guess: u32 = guess.trim().parse() { // compile error
            Ok(num) => num,
            Err(_) => continue,
        };

compile error:

    >>> cargo run 
       Compiling guessing_game v0.1.0 (file:///Users/JaySon/Projects/RustProjects/guessing_game)
    src/main.rs:22:47: 22:48 error: expected one of `.`, `;`, or an operator, found `{`
    src/main.rs:22         let guess: u32 = guess.trim().parse() {
                                                                 ^
    Could not compile `guessing_game`.

    To learn more, run the command again with --verbose.

miss a match before that. fix:

        let guess: u32 = match guess.trim().parse() {
            // ...
        };
