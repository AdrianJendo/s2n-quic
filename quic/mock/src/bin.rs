extern crate mock; // not needed since Rust edition 2018

use mock::seamock;

#[seamock]
pub trait test {
    fn a(&self) -> bool;
    fn b(&self) -> u8;
    fn c(&self) -> i32;
}


pub fn main() {
    // test();
}