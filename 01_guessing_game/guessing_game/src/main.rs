// importing io for interacting with the user
// io is part of the std library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello pal, guess my number.");
    
    let secret = rand::thread_rng().gen_range(1,101);
    
    // Uncomment for debugging
    // println!("The secret number is {}", secret);

    //loop is a loop with no condition.
    loop {
        println!("Please input your guess");
        // all variables are immutable by default, to mutate them, use let mut
        // String::new() creates an empty String object.
        // :: means associated function, associated functions are part of the class
        // itself, and not of specific instances.
        let mut guess = String::new();
        
        //references are immutable by default.
        // &guess would be immutable
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("you guessed {}", guess);

        //Trimming eliminates whitespace (lead and trail)
        //Parse needs a defined data type (unsigned int 32 bit here)
        // option 1:
        // let guess: u32 = guess.trim().parse().expect("Please enter an integer");

        //option 2: avoid crash
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                num
            }
            Err(_) => {
                println!("insert a valid value");
                continue;
            }
        };
        

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("You're too small");
            }
            Ordering::Greater => {
                println!("Too high");
            }
            Ordering::Equal => {
                println!("Good gal");
                break;
            }

        }
    }
}
