//! <!-- cargo-sync-readme -->
//! Extensions to `tokio::process::Child` to terminate processes.
//!
//! ```rust
//! use tokio::process::Command;
//! use tokio_process_terminate::TerminateExt;
//!
//! #[tokio::main]
//! async fn main() {
//! #   #[cfg(unix)]
//! #   {
//!     let mut command = Command::new("sleep")
//!         .arg("10")
//!         .spawn()
//!         .unwrap();
//!     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//!     let exit = command.terminate_wait().await.unwrap();
//!     dbg!(exit);
//!     let code = exit.code();
//!     // On Unix, code should be `None` if the process was terminated by a signal.
//!     assert!(code.is_none());
//! #   }
//! }
//! ```

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
    ///
    /// If the process exits before the timeout, the exit status is returned.
    /// If the timeout elapses before the process exits, it is killed and `None` is returned.
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
