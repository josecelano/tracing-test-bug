extern crate tracing;

pub mod app;

#[cfg(test)]
mod tests {
    use std::thread;
    use tracing::info;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_logs_are_not_captured_with_std_threads() {
        info!("level 1");

        let handle_1 = thread::spawn(move || {
            info!("level 2");

            let handle_2 = thread::spawn(move || {
                info!("level 3");
            });

            handle_2.join().unwrap();
        });

        handle_1.join().unwrap();

        assert!(logs_contain("level 1"));
        assert!(!logs_contain("level 2"));
        assert!(!logs_contain("level 3"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_logs_are_captured_with_tokio() {
        info!("level 1");

        tokio::spawn(async {
            info!("level 2");

            tokio::spawn(async {
                info!("level 3");
            })
            .await
            .unwrap();
        })
        .await
        .unwrap();

        assert!(logs_contain("level 1"));
        assert!(logs_contain("level 2"));
        assert!(logs_contain("level 3"));
    }
}
