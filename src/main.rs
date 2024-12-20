use tracing_test_bug::app;

fn main() {
    let init_subscriber = true;
    app::run(init_subscriber)
}
