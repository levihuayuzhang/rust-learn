use adder::add_two;

mod common;
// use common::setup;

const fn fib(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut i = 0;
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    a
}

const FIB_10: u64 = fib(10);

#[test]
fn it_add_two() {
    common::setup();
    // setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}

#[test]
fn fib_ten() {
    assert_eq!(FIB_10, 55);
}
