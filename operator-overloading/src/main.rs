#![allow(unused_mut)]
#![allow(unused_variables)]

use std::ops::{Add};

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}

impl<T> Add for Complex<T> where T: Add<Output = T> {
    type Output = Complex<T>;

    // a + b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }

    }
}

fn main() {
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(3, 4);
    println!("{:?}", a + b);
}
