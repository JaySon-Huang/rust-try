use std::thread;
// use `Mutex` to control concurrency
// use `Arc<T>`
use std::sync::{Mutex, Arc};

// represent a philosopher
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

// represent a table
struct Table {
    // forks is a vector of `Mutex`es
    // use an empty tuple, `()`, inside the mutex
    forks: Vec<Mutex<()>>,
}

// define things on Philosopher structs
impl Philosopher {
    // argument: name, of type &str
    // return: an instance of Philosopher struct
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        // create a new Philosopher
        Philosopher {
            // set its name
            name: name.to_string(), // to_string return a copy of name(&str)
            left: left,
            right: right,
        }
        // the last expression is automatically returned
    }

    fn eat(&self, table: &Table) {
        // `lock` on left and right forks
        // adding `_` means that we accquire them but not use the value
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating...", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating", self.name);

        // when `_left` and `_right` go out of scope,
        //   Rust automatically release the lock.
    }
}

fn main() {
    // `arc` stands for `atomic reference count`
    let table = Arc::new(
        Table {
            forks: vec![
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
            ]
        }
    );

    // define a vector, Vec<T>
    let philosophers = vec![
        Philosopher::new("A", 0, 1),
        Philosopher::new("B", 1, 2),
        Philosopher::new("C", 2, 3),
        Philosopher::new("D", 3, 4),
        // Philosopher::new("E", 4, 0),
        Philosopher::new("E", 0, 4), // prevent deadlock
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
        // ?
        let table = table.clone();

        // `thread::spawn` takes a closure as an argument
        //   and executes it in a new thread.
        // `move` means the closure take ownership of the values
        //   it's capturing
        thread::spawn(move || {
            // call `eat` on p
            p.eat(&table);
        })
    }).collect(); // collect the result of `map` calls

    for h in handles {
        // `join` block execution until the thread has completed execution
        h.join().unwrap();
    }
}
