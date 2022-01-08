#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rand::Rng;
use std::io::stdin;

fn main() {
    // Generate a number between 1 and 100
    let number = rand::thread_rng().gen_range(1, 101); // 101 not inclusive
    loop {
        println!("Enter your number guess:");
        
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!! Thanks for playing");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read your input. {}", e)
                    }
                }
            },
            Err(_) => continue,
        }
    }
}
