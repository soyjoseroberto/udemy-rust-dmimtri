#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

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
}
