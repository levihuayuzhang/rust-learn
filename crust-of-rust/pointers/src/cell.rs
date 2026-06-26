use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

// #[cfg(test)]
// mod test {
//     use super::Cell;

//     #[test]
//     fn bad() {
//         use std::sync::Arc;
//         let x = Arc::new(Cell::new([0; 10240]));

//         let x1 = Arc::clone(&x);
//         let jh1 = std::thread::spawn(move || {
//             x1.set([1; 10240]);
//         });

//         let x2 = Arc::clone(&x);
//         let jh2 = std::thread::spawn(move || {
//             x2.set([2; 10240]);
//         });
//         jh1.join().unwrap();
//         jh2.join().unwrap();
//         let xs = x.get();
//         for &i in xs.iter() {
//             eprintln!("{}", i);
//         }
//     }

//     #[test]
//     fn bad2() {
//         let x = Cell::new(String::from("hello"));
//         let first = x.get();
//         x.set(String::new());
//         x.set(String::from("world"));
//         eprintln!("{}", first);
//     }
// }
