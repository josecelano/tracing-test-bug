use std::thread;
use tracing::{info, level_filters::LevelFilter};

pub fn run(init_subscriber: bool) {
    if init_subscriber {
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_ansi(true)
            .init();
    }

    info!("level 1");

    let handle_1 = thread::spawn(move || {
        info!("level 2");

        let handle_2 = thread::spawn(move || {
            info!("level 3");
        });

        handle_2.join().unwrap();
    });

    handle_1.join().unwrap();
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::app;

    #[tokio::test]
    #[traced_test]
    async fn test_logs_are_captured_with_tokio() {
        tokio::spawn(async {
            let init_subscriber = false;
            app::run(init_subscriber);
        })
        .await
        .unwrap();

        assert!(logs_contain("level 1"));
        assert!(!logs_contain("level 2"));
        assert!(!logs_contain("level 3"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_logs_are_not_captured_with_tokio_spawn_blocking() {
        tokio::task::spawn_blocking(move || {
            let init_subscriber = false;
            app::run(init_subscriber);
        })
        .await
        .unwrap();

        assert!(!logs_contain("level 1"));
        assert!(!logs_contain("level 2"));
        assert!(!logs_contain("level 3"));
    }
}
