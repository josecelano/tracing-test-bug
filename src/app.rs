use tracing::{info, level_filters::LevelFilter};

pub async fn run(init_subscriber: bool) {
    if init_subscriber {
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_ansi(true)
            .init();
    }

    info!("level 1");

    let future1 = async move {
        info!("level 2");

        let future2 = async move {
            info!("level 3");
        };

        let handle_3 = tokio::spawn(future2);

        handle_3.await.unwrap();
    };

    let handle_1 = tokio::spawn(future1);

    handle_1.await.unwrap();
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::app;

    #[tokio::test]
    #[traced_test]
    async fn test_logs_are_captured_with_tokio() {
        tokio::spawn(app_run_without_initializing_tracing())
            .await
            .unwrap();

        assert!(logs_contain("level 1"));
        assert!(logs_contain("level 2"));
        assert!(logs_contain("level 3"));
    }

    async fn app_run_without_initializing_tracing() {
        let init_subscriber = false;
        app::run(init_subscriber).await;
    }
}
