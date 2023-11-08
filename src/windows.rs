use std::process::ExitStatus;

use super::TerminateExt;

#[async_trait::async_trait]
impl TerminateExt for tokio::process::Child {
    fn terminate(&mut self) {
        if let Some(pid) = self.id() {
            unsafe {
                windows_sys::Win32::System::Console::GenerateConsoleCtrlEvent(
                    windows_sys::Win32::System::Console::CTRL_C_EVENT,
                    pid as _,
                );
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_terminate_sleep() {
        let mut command = tokio::process::Command::new("timeout")
            .args(["/t", "10", "/nobreak"])
            .spawn()
            .unwrap();
        let instant = std::time::Instant::now();
        tokio::time::sleep(Duration::from_secs(1)).await;

        command.terminate_wait().await.unwrap();
        assert!(instant.elapsed() < Duration::from_secs(2));
    }
}
