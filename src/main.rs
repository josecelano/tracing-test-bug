use tracing_test_bug::app;

#[tokio::main]
async fn main() {
    let init_subscriber = true;
    app::run(init_subscriber).await;
}
