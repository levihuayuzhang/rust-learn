use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    signal,
    sync::broadcast,
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);
    let token = CancellationToken::new();
    let cancel_token = token.clone();

    tokio::spawn(async move {
        tracing::info!("Spawning new task");
        match signal::ctrl_c().await {
            Ok(()) => {
                tracing::warn!("Shutdown Tasks");
                cancel_token.cancel();
            }
            Err(err) => {
                tracing::error!("Error: {err:#?}");
            }
        }
    });

    loop {
        let token = token.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut socket, address) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (stream_reader, mut stream_writer) = socket.split();
            let mut message = String::new();
            let mut reader = BufReader::new(stream_reader);

            loop {
                tokio::select! {
                    // pattern = future => handler
                    result = reader.read_line(&mut message) => {
                        tracing::info!("Received message from client: {}", &message);
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((message.clone(), address)).unwrap();
                        message.clear();
                    }
                    result = rx.recv() => {
                        let (received_message, sender_address) = result.unwrap();
                        if address != sender_address {
                            tracing::info!("Received message from channel: {}", &message);
                            stream_writer
                                .write_all(received_message.as_bytes())
                                .await
                                .unwrap();
                            }
                    }
                    _ = token.cancelled() => {
                        // println!("Cleaning up...");
                        tracing::info!("Cleaning up...");
                        return;
                    }
                }
            }
        });
    }
}
