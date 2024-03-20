extern crate mock; // not needed since Rust edition 2018

use mock::seamock;

#[seamock]
pub trait Test {
    fn a(&self, _z: bool) -> bool;
    fn b(&self) -> u8;
    fn c(&self) -> i32;
}


pub fn main() {
    let x = MockTest::new();

    x.a(false, false);
    x.a(false, false);
    x.b(1);
    x.c(32);

    assert!(x.expect_times_a(2));
    assert!(x.expect_times_b(1));
    assert!(x.expect_times_c(1));
}