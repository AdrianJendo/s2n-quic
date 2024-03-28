extern crate mock; // not needed since Rust edition 2018

use mock::seamock;

#[seamock]
pub trait Test {
    fn a(&self, z: bool, a: i32) -> bool;
    fn b(&self) -> u8;
    fn c(&self) -> i32;
}

enum WithVal<T> {
    Gt(T),
    Gte(T),
    Lt(T),
    Lte(T),
    Eq(T),
}

struct Tmp {
    max_times_a: u64,
    max_times_b: u64,
    max_times_c: u64,
    times_a: std::cell::RefCell<u64>,
    times_b: std::cell::RefCell<u64>,
    times_c: std::cell::RefCell<u64>,
    val_returning_a: fn(z:bool, a:i32) -> bool,
    val_returning_b: fn() -> u8,
    val_returning_c: fn() -> i32,
    val_with_a: Option<(WithVal<bool>, WithVal<i32>)>,
}

impl Tmp {
    pub fn new() -> Self {
        Self {
            max_times_a: u64::MAX,
            max_times_b: u64::MAX,
            max_times_c: u64::MAX,
            times_a: std::cell::RefCell::new(0),
            times_b: std::cell::RefCell::new(0),
            times_c: std::cell::RefCell::new(0),
            val_returning_a: |z:bool, a:i32| Default::default(),
            val_returning_b: || Default::default(),
            val_returning_c: || Default::default(),
            val_with_a: None,
        }
    }

    fn returning_a(&mut self, f: fn(z: bool, a: i32) -> bool) -> &mut Self {
        self.val_returning_a = f;
        self
    }
    fn returning_b(&mut self, f: fn() -> u8) -> &mut Self {
        self.val_returning_b = f;
        self
    }

    fn returning_c(&mut self, f: fn() -> i32) -> &mut Self {
        self.val_returning_c = f;
        self
    }

    fn with_a(&mut self, with: (WithVal<bool>, WithVal<i32>)) -> &mut Self {
        self.val_with_a = Some(with);
        self
    }

    fn times_a(&mut self, val: u64) -> &mut Self {
        self.max_times_a = val;
        self
    }

    fn times_b(&mut self, val: u64) -> &mut Self {
        self.max_times_b = val;
        self
    }

    fn expect_times_a(&self, val: u64) -> bool {
        *self.times_b.borrow() == val
    }

    fn expect_times_b(&self, val: u64) -> bool {
        *self.times_b.borrow() == val
    }

    fn a(&self, z: bool, a: i32) -> bool {
        self.times_a.replace_with(|&mut old| old + 1);
        if *self.times_a.borrow() > self.max_times_a {
            panic!("a called more than {} times", self.max_times_a)
        }
        // match self.val_with_a {
        //     Some(WithVal::Gt(val)) => {
        //         assert!(val > z);
        //     }
        //     Some(WithVal::Eq(val)) => {
        //         assert_eq!(val, z);
        //     }
        //     None => {}
        //     _ => {}
        // }
        (self.val_returning_a)(z, a)
    }
    fn b(&self) -> u8 {
        self.times_b.replace_with(|&mut old| old + 1);
        if *self.times_b.borrow() > self.max_times_b {
            panic!("b called more than {} times", self.max_times_b)
        }
        (self.val_returning_b)()
    }
    fn c(&self) -> i32 {
        self.times_c.replace_with(|&mut old| old + 1);
        if *self.times_c.borrow() > self.max_times_c {
            panic!("b called more than {} times", self.max_times_c)
        }
        (self.val_returning_c)()
    }
}


pub fn main() {
    let mut x = MockTest::new();

    x
        .times_a(1)
        .with_a((WithVal::Eq(true), WithVal::Eq(2)))
        .returning_a(|x, y| !x)
        .returning_b(|| 4)
        .times_b(2);

    assert_eq!(false, x.a(true, 1));
    assert_eq!(4, x.b());
    assert_eq!(4, x.b());
    assert!(x.expect_times_b(2));
    assert_ne!(x.expect_times_b(3), true);


    // let x = MockTest::new();
    //
    // x.a(false, false);
    // x.a(false, false);
    // x.b(1);
    // x.c(32);
    //
    // assert!(x.expect_times_a(2));
    // assert!(x.expect_times_b(1));
    // assert!(x.expect_times_c(1));
}