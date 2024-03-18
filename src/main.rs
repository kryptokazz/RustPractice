// Define a custom error type
#[derive(Debug)]
enum CustomError {
    InvalidInput,
    CalculationError,
}

// Define a function that returns a Result type
fn divide(x: i32, y: i32) -> Result<f64, CustomError> {
    if y == 0 {
        return Err(CustomError::InvalidInput);
    }
    Ok(x as f64 / y as f64)
}

// Define a module with a function
mod greetings {
    pub fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
}

// Define a struct
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Define a method for the struct
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    // Method to print the person's information
    fn print_info(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

// Define an enum representing different types of animals
enum Animal {
    Dog,
    Cat,
    Bird(String),
}

fn main() {
    // Variables and data types
    let a: i32 = 10;
    let b: f64 = 3.5;
    let c = "Rust";

    // Printing variables
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);

    // Function call
    greetings::greet("World");

    // Looping
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }

    // Conditional statement
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result of division: {}", value),
        Err(err) => println!("Error: {:?}", err),
    }

    // Using if-else
    let number = 42;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // Creating an instance of Person struct
    let person = Person::new("Alice", 30);
    person.print_info();

    // Example of pattern matching with enums
    let animal = Animal::Bird(String::from("Sparrow"));
    match animal {
        Animal::Dog => println!("It's a dog!"),
        Animal::Cat => println!("It's a cat!"),
        Animal::Bird(name) => println!("It's a bird: {}", name),
    }

    // Example of ownership and borrowing
    let mut s = String::from("hello");
    change(&mut s);
    println!("The modified string is: {}", s);
}

// Function that takes ownership of a String and modifies it
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

