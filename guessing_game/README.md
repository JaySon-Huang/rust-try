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

* * *

