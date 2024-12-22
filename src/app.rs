use tracing::{info, level_filters::LevelFilter};

pub async fn run(init_subscriber: bool) {
    if init_subscriber {
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_ansi(true)
            .init();
    }

    info!("level 1");

    let handle_1 = tokio::spawn(async move {
        info!("level 2");

        let handle_2 = tokio::spawn(async move {
            info!("level 3");
        });

        handle_2.await.unwrap();
    });

    handle_1.await.unwrap();
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::app;

    #[tokio::test]
    #[traced_test]
    async fn test_logs() {
        tokio::spawn(async {
            let init_subscriber = false;
            app::run(init_subscriber).await;
        })
        .await
        .unwrap();

        assert!(logs_contain("level 1"));
        assert!(logs_contain("level 2"));
        assert!(logs_contain("level 3"));
    }
}
