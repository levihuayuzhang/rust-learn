// use std::{thread, time::Duration};

use std::{thread, time::Duration};

fn main() {
    // let mut handles = Vec::new();

    // for i in 0..5 {
    //     let handle = thread::spawn(move || {
    //         thread::sleep(Duration::from_secs(1));
    //         println!("Normal thread: {i}")
    //     });
    //     handles.push(handle);
    // }

    // handles.into_iter().for_each(|h| h.join().unwrap());

    let a = String::from("Helllo");

    thread::scope(|s| {
        let b = String::from("World");

        for i in 0..5 {
            // s.spawn(|| {
            s.spawn(move || {
                thread::sleep(Duration::from_secs(1));
                // println!("Scoped thread: {a}");
                // println!("Scoped thread: {b}");
                println!("Scoped thread: {i}");
            });
        }
    })
}
