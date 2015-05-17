use std::thread;

// represent a philosopher
struct Philosopher {
    name: String,
}

// define things on Philosopher structs
impl Philosopher {
    // argument: name, of type &str
    // return: an instance of Philosopher struct
    fn new(name: &str) -> Philosopher {
        // create a new Philosopher
        Philosopher {
            // set its name
            name: name.to_string(), // to_string return a copy of name(&str)
        }
        // the last expression is automatically returned
    }

    fn eat(&self) {
        println!("{} is eating...", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating", self.name);
    }
}

fn main() {
    // define a vector, Vec<T>
    let philosophers = vec![
        Philosopher::new("A"),
        Philosopher::new("B"),
        Philosopher::new("C"),
        Philosopher::new("D"),
        Philosopher::new("E"),
    ];

    // iterate the philosophers, Vec<T>
    // for p in &philosophers {
    //     p.eat();
    // }

    /* operating in a concurrent fashion */

    // `handles` is a `Vec` of handle of threads,
    // `Vec<_>` let Rust figure out what type in the Vec
    // `into_iter` create an iterator, then call `map` on it
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // `thread::spawn` takes a closure as an argument
        //   and executes it in a new thread.
        // `move` means the closure take ownership of the values
        //   it's capturing
        thread::spawn(move || {
            // call `eat` on p
            p.eat();
        })
    }).collect(); // collect the result of `map` calls

    for h in handles {
        // `join` block execution until the thread has completed execution
        h.join().unwrap();
    }
}
