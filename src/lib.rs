use std::{process::ExitStatus, time::Duration};

#[async_trait::async_trait]
pub trait TerminateExt {
    /// Send a signal to the process to terminate it.
    ///
    /// On Unix, this sends a SIGTERM signal to the process.
    /// On Windows, this sends a CTRL_C_EVENT to the process.
    fn terminate(&mut self);

    #[doc(hidden)]
    async fn _wait(&mut self) -> std::io::Result<ExitStatus>;
    #[doc(hidden)]
    async fn _kill(&mut self) -> std::io::Result<()>;

    /// Terminate the process and wait for it to exit.
    async fn terminate_wait(&mut self) -> std::io::Result<ExitStatus> {
        self.terminate();
        self._wait().await
    }

    /// Terminate the process and wait for it to exit, or kill it after a timeout.
    async fn terminate_timeout(
        &mut self,
        timeout: Duration,
    ) -> std::io::Result<Option<ExitStatus>> {
        self.terminate();
        match tokio::time::timeout(timeout, self._wait()).await {
            Ok(result) => result.map(Some),
            Err(_) => self._kill().await.map(|_| None),
        }
    }
}

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_terminate_sleep() {
        let mut command = tokio::process::Command::new("sleep")
            .arg("10")
            .spawn()
            .unwrap();
        let instant = std::time::Instant::now();
        tokio::time::sleep(Duration::from_secs(1)).await;

        command.terminate_wait().await.unwrap();
        assert!(instant.elapsed() < Duration::from_secs(2));
    }
}
