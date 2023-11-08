# tokio-process-terminate

<!-- cargo-sync-readme start -->

<!-- cargo-sync-readme -->
Extensions to `tokio::process::Child` to terminate processes.

```rust
use tokio::process::Command;
use tokio_process_terminate::TerminateExt;

#[tokio::main]
async fn main() {
    let mut command = Command::new("sleep")
        .arg("10")
        .spawn()
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let exit = command.terminate_wait().await.unwrap();
    dbg!(exit);
    let code = exit.code();
    // On Unix, code should be `None` if the process was terminated by a signal.
    assert!(code.is_none());
}
```

<!-- cargo-sync-readme end -->

