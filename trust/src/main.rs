use std::io::prelude::*;
use std::{io, thread};

fn main() -> io::Result<()> {
    let mut i = trust::Interface::new()?;
    let mut l = i.bind(7000)?;
    let jh = thread::spawn(move || {
        while let Ok(mut stream) = l.accept() {
            eprintln!("got connection!");

            let n = stream.read(&mut [0]).unwrap();
            eprintln!("read data");
            assert_eq!(n, 0);
        }
    });

    jh.join().unwrap();
    Ok(())
}
