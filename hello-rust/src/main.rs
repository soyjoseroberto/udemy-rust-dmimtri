#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
mod pattern_matching;
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


// Structs
struct Point {
    x: f64,
    y: f64,
}

// A struct that containes a struct
struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    // Instantiate a struct
    let p1 = Point { x: 3.0, y: 4.0 };
    println!("Point p1 is at ({}, {})", p1.x, p1.y);

    // A second point
    let p2 = Point { x: 5.0, y: 10.0 };

    // Now create a line with 2 points
    let my_line = Line { start: p1, end: p2 };
    println!("Line from {} to {}", my_line.start.x, my_line.end.y);
}

fn vectors() {
    /*
    let mut a = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);
    */

    let mut a = vec![1, 2, 3]; // [1;10]
    println!("a = {:?}", a);

    let idx/*:i32*/ = 0; // will not work with :i32
                         // you need usize
    println!("a[0] = {}", a[idx]);

    // unsafe access
    //println!("a[5] = {}", a[5]);

    match a.get(5) {
        Some(x) => println!("a[5] = {}", x),
        None => println!("error, no such element"),
    }

    // iterate
    for x in &a {
        println!("{}", x);
    }

    // adding/removing
    a.push(44);
    println!("{:?}", a);

    let last_elem = a.pop(); // can easily yield nothing
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // explain why this doesn't work
    //let Some(last_value) = a.pop();

    // print the elements in reverse order
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn iterators() {
    let number_vec = vec![3, 2, 1];

    // use iter() to iterate
    for x in number_vec.iter() {
        println!("Vector item: {}", x); // Same result with *x, follow the ref to x.
    }
    
    // Print vector in reverse
    for y in number_vec.iter().rev() {
        println!("Vector in reverse: {}", y); // Same result with *x, follow the ref to x.
    }

    // Using a mutable vector
    let mut numbers = vec![3, 2, 1];

    // use iter_mut()S
    for x in numbers.iter_mut() {
        println!("Vector mut item: {}", *x + 1); // Must use *x here
    }

    // into_iter() is used by extend()
    let mut more_numbers = vec![9, 8, 7];
    more_numbers.extend(numbers); // cannot use numbers vector after this line
    println!("{:?}", more_numbers); // prints [9, 8, 7, 3, 2, 1]




}

// Closures
fn say_hello() { println!("hello"); } // fn ref can be copied in closure
fn closures() {
    let sh = say_hello; // Not invoking say_hello
    sh(); // Now invoking say_hello() through sh variable

    // Closure, look at the |x| signature
    let plus_one = |x: i32| -> i32 { x + 1 }; // Do not add ; after x + 1
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

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

    structures();

    // Function from an external file
    pattern_matching::demo();

    // Vectors
    vectors();

    // Iterators
    iterators();

    // Closures
    closures();
}
