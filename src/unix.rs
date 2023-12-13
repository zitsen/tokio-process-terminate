use std::process::ExitStatus;

use super::TerminateExt;

#[async_trait::async_trait]
impl TerminateExt for tokio::process::Child {
    fn terminate(&mut self) {
        if let Some(pid) = self.id() {
            unsafe { libc::kill(pid as _, libc::SIGTERM) };
        }
    }
    #[doc(hidden)]
    async fn _wait(&mut self) -> std::io::Result<ExitStatus> {
        self.wait().await
    }
    #[doc(hidden)]
    async fn _kill(&mut self) -> std::io::Result<()> {
        self.kill().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait TerminatePgExt {
    /// Send a signal to the process group to terminate it.
    ///
    /// Unix only, this sends a SIGTERM signal to the process group.
    fn terminate_pg(&mut self);

    #[doc(hidden)]
    async fn _wait_pg(&mut self) -> std::io::Result<ExitStatus>;
    #[doc(hidden)]
    async fn _kill_pg(&mut self) -> std::io::Result<()>;

    /// Terminate the process group and wait for it to exit.
    async fn terminate_pg_wait(&mut self) -> std::io::Result<ExitStatus> {
        self.terminate_pg();
        self._wait_pg().await
    }

    /// Terminate the process group and wait for it to exit, or kill it after a
    /// timeout.
    ///
    /// If the process group exits before the timeout, the exit status is
    /// returned. If the timeout elapses before the process group exits, it is
    /// killed and `None` is returned.
    async fn terminate_pg_timeout(
        &mut self,
        timeout: std::time::Duration,
    ) -> std::io::Result<Option<ExitStatus>> {
        self.terminate_pg();
        match tokio::time::timeout(timeout, self._wait_pg()).await {
            Ok(result) => result.map(Some),
            Err(_) => self._kill_pg().await.map(|_| None),
        }
    }
}

#[async_trait::async_trait]
impl TerminatePgExt for tokio::process::Child {
    fn terminate_pg(&mut self) {
        if let Some(pid) = self.id() {
            unsafe { libc::killpg(pid as _, libc::SIGTERM) };
        }
    }
    #[doc(hidden)]
    async fn _wait_pg(&mut self) -> std::io::Result<ExitStatus> {
        self.wait().await
    }
    #[doc(hidden)]
    async fn _kill_pg(&mut self) -> std::io::Result<()> {
        if let Some(pid) = self.id() {
            unsafe { libc::killpg(pid as _, libc::SIGKILL) };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

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
