// https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=13

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // x86: CAS
        // ARM: LDREX STREX
        //  - compare_exchange: impl using a loop of LDREX and STREX
        //  - compare_exchange_weak: LDREX STREX
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol: stay in S when locked
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                thread::yield_now();
            }
            thread::yield_now();
        }
        // SAFETY: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

use std::thread::{self, spawn};
#[test]
fn mutex_test() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles: Vec<_> = (0..100)
        .map(|_| {
            spawn(move || {
                for _ in 0..1000 {
                    l.with_lock(|v| {
                        *v += 1;
                    });
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(l.with_lock(|v| *v), 100 * 1000);
}

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
}

fn main() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::Release);
    });
    let _ty = spawn(move || {
        y.store(true, Ordering::Release);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) {}
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);
}
