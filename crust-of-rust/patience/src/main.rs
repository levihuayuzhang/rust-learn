#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    println!("Hello, world!");

    let x = foo1();
}

async fn foo1() -> usize {
    println!("foo");
    0
}

async fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo1");
        foo1().await;
        println!("foo2");

        0
    }
}
