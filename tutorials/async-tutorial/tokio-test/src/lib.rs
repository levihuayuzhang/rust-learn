async fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        assert_eq!(rt.block_on(double(2)), 4);
    }

    #[tokio::test]
    // #[tokio::test(flavor = "current_thread")]
    // #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_double_tokio() {
        assert_eq!(double(2).await, 4);
    }
}
