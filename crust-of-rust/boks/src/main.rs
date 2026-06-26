// https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8

#![feature(dropck_eyepatch)]

use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Boks<T> {
    p: NonNull<T>,
    _t: PhantomData<T>,
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.p.as_mut());
        };
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            // SAFETY: box never creates a null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turn into a ponter
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turn into a ponter
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        // Also, since we have &mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p.as_mut() }
    }
}

use std::fmt::Debug;
struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        // println!("{:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::ny(&mut y);
    // let b = Box::new(&mut y);
    println!("{:?}", y);

    let mut z = 42;
    // let b = Boks::ny(Oisann(&mut z));
    // let b = Box::new(Oisann(&mut z));
    println!("{:?}", z);
    // drop(b);

    let s = String::from("hei");
    let mut box1 = Box::new(&*s);
    let box2: Box<&'static str> = Box::new("heisann");
    box1 = box2;

    let s = String::from("hei");
    let mut boks1 = Boks::ny(&*s);
    let boks2: Boks<&'static str> = Boks::ny("heisann");
    boks1 = boks2;
}
