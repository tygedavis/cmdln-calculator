mod engine;

use std::io;

fn main() {
    loop {
        println!("Please type an input: ");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Cannot read input");

        let calculation = match engine::parse(&user_input) {
            Ok(calc) => calc, // On success, get the Calculation struct out
            Err(e) => {
                // On failure, print the error and exit
                println!("Parsing Error: {}", e);
                return;
            }
        };

        match engine::evaluate(&calculation) {
            Ok(num) => println!("Your answer is {}", num),
            Err(e) => println!("There was an error: {}", e)
        }
    }
}
