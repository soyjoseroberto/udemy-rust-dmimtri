extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen();
    println!("Random number generated: {}", random_number);
}
