use std::process::ExitStatus;

use windows_sys::Win32::System::Threading::TerminateProcess;
use windows_sys::Win32::UI::WindowsAndMessaging::{PostThreadMessageA, SendMessageW, WM_CLOSE};

use super::TerminateExt;

#[async_trait::async_trait]
impl TerminateExt for tokio::process::Child {
    fn terminate(&mut self) {
        if let Some((pid, hwd)) = self.id().zip(self.raw_handle()) {
            unsafe {
                if PostThreadMessageA(pid, WM_CLOSE, 0, 0) != 0 {
                    return;
                }
                if SendMessageW(hwd as _, WM_CLOSE, 0, 0) != 0 {
                    return;
                }
                if TerminateProcess(hwd as _, 1) != 0 {
                    return;
                }
            }
        }
    }

    #[doc(hidden)]
    async fn _wait(&mut self) -> std::io::Result<ExitStatus> {
        self.wait().await
    }
    #[doc(hidden)]
    async fn _kill(&mut self) -> std::io::Result<()> {
        self.kill().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    async fn sleep() {
        let instant = std::time::Instant::now();
        let mut command = tokio::process::Command::new("timeout")
            .args(["/t", "20"])
            .spawn()
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;

        command.terminate_wait().await.unwrap();
        assert!(dbg!(instant.elapsed()) < Duration::from_secs(5));
        println!("terminated");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    #[tokio::test]
    async fn test_terminate_sleep() {
        let mut ctrl_break = tokio::signal::windows::ctrl_break().unwrap();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                panic!("parent terminated");
            }
            _ = ctrl_break.recv() => {
                panic!("parent ctrl-break")
            }
            _ = sleep() => {
                println!("sleep terminated");
            }
        }
    }
}
