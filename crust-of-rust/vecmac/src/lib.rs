// https://youtu.be/q6paRBbLgNw?si=IBrq6y7QF6yLk_n-
// https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd45a

#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        // check that count is const
        const C: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C);
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => {()};
}

/* trait MaxValue {
    fn max_value() -> Self;
}
macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}
max_impl!(i32);
max_impl!(u32);
max_impl!(i64);
max_impl!(u64); */

#[cfg(test)]
mod test {
    #[test]
    fn empty_vec() {
        let x: Vec<u32> = avec![];
        assert!(x.is_empty());
    }

    #[test]
    fn single() {
        let x: Vec<u32> = avec![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn double() {
        let x: Vec<u32> = avec![42, 43];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 43);
    }

    #[test]
    fn trailing() {
        let _: Vec<&'static str> = avec![
            "laskdjsadfkdjasfalsd;kfjkals;djfklasdjfaklsdjffjdslakfjl",
            "laskdjsadfkdjasfalsd;kfjkals;djfklasdjfaklsdjffjdslakfjl",
            "laskdjsadfkdjasfalsd;kfjkals;djfklasdjfaklsdjffjdslakfjl",
            "laskdjsadfkdjasfalsd;kfjkals;djfklasdjfaklsdjffjdslakfjl",
            "laskdjsadfkdjasfalsd;kfjkals;djfklasdjfaklsdjffjdslakfjl",
        ];
    }
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![42; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

/// compile fail test
/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
///
/// compile success test
/// ```
/// let x: Vec<u32> = vecmac::avec![42; 42];
/// ```
#[allow(dead_code)]
pub struct CompileFailTest;
