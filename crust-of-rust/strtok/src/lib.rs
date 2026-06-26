// https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=13

pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn check_is_static(_: &'static str) {}

        let mut x = "hello world";
        check_is_static(x);

        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}
