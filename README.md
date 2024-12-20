# Tracing Test Bug

This repo was created to reproduce a bug in the [tracing-test](https://github.com/dbrgn/tracing-test) crate.

When you run the application, you should see the following output:

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/tracing-test-bug`
2024-12-20T09:56:43.725466Z  INFO tracing_test_bug: level 1
2024-12-20T09:56:43.725551Z  INFO tracing_test_bug: level 2
2024-12-20T09:56:43.725650Z  INFO tracing_test_bug: level 3
```

The application has three levels of nested threads.

When you run the tests, logs are apparently only captured in the same thread.

## Links

- Issue on [tracing-test](https://github.com/dbrgn/tracing-test) repo: [No capture from std::thread::spawn or tokio::task::spawn_blocking](<https://github.com/dbrgn/tracing-test/issues/23>).
