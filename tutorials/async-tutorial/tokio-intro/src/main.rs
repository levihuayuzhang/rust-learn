use tokio::runtime;

async fn hi() {
    println!("Hello Tokio!");
}

// #[tokio::main(flavor = "current_thread")]
// #[tokio::main()]
// async fn main() {
fn main() {
    // let rt = runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();
    // rt.block_on(hi());

    // hi().await;

    let rt = runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .thread_stack_size(5 * 1024 * 1024)
        .event_interval(20)
        .max_blocking_threads(256)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(hi());
}
