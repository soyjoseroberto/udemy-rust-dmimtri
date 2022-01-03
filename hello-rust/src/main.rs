#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn operators()
{
    println!("Operators section");
    // Arithmetic
    let mut a = 2 + 3 * 4; // +- */ as other languages, precedence applies
    a = a + 1; // No ++ or -- operators, have to do it like this
    a -= 2; // Rust does have += -= *= %= operators

    println!("a = {}", a);

    // No exp operator like ^ in other languages, use built-in fn instead
    let b = 2.5; // f64 by type inference 
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("b_cubed = {}, b_to_pi = {}", b_cubed, b_to_pi);
}

fn main() {
    println!("Hello, world!");

    let a: u8 = 123; // unsigned, 8 bits, 0 - 255
    println!("a = {}", a); // immutable by default

    let mut b: i8 = 50; // mut makes it mutable
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    // Type inference
    let c = 123456789; // Output: 4 bytes = 32 bits = i32 type
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    operators();
}
