async fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        let result = double(2);
        assert_eq!(result, 4);
    }
}
