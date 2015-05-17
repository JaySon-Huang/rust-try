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
    for p in &philosophers {
        p.eat();
    }
}
