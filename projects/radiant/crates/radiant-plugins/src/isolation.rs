use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IsolationPolicy {
    pub timeout: Duration,
    pub latency_frames: u32,
}

impl Default for IsolationPolicy {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(50),
            latency_frames: 256,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IsolationState {
    Stopped,
    Running,
    Crashed,
    TimedOut,
}

#[derive(Debug, Error)]
pub enum IsolationError {
    #[error("worker failed to start: {0}")]
    Spawn(String),
    #[error("worker crashed with status {0:?}")]
    Crashed(Option<i32>),
    #[error("worker exceeded isolation timeout")]
    TimedOut,
}

/// Supervises a plugin worker process. Audio IPC is intentionally explicit at this boundary;
/// a failed worker can only mute/recover a slot and cannot terminate the DAW process.
pub struct ProcessingIsolation {
    policy: IsolationPolicy,
    state: IsolationState,
}

impl ProcessingIsolation {
    #[must_use]
    pub fn new(policy: IsolationPolicy) -> Self {
        Self {
            policy,
            state: IsolationState::Stopped,
        }
    }
    #[must_use]
    pub const fn state(&self) -> IsolationState {
        self.state
    }
    #[must_use]
    pub const fn latency_frames(&self) -> u32 {
        self.policy.latency_frames
    }

    pub fn run_worker(
        &mut self,
        program: impl AsRef<Path>,
        args: &[&str],
    ) -> Result<(), IsolationError> {
        self.state = IsolationState::Running;
        let mut child = Command::new(program.as_ref())
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| {
                self.state = IsolationState::Crashed;
                IsolationError::Spawn(error.to_string())
            })?;
        let started = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(status)) if status.success() => {
                    self.state = IsolationState::Stopped;
                    return Ok(());
                }
                Ok(Some(status)) => {
                    self.state = IsolationState::Crashed;
                    return Err(IsolationError::Crashed(status.code()));
                }
                Ok(None) if started.elapsed() >= self.policy.timeout => {
                    let _ = child.kill();
                    let _ = child.wait();
                    self.state = IsolationState::TimedOut;
                    return Err(IsolationError::TimedOut);
                }
                Ok(None) => thread::sleep(Duration::from_millis(1)),
                Err(error) => {
                    self.state = IsolationState::Crashed;
                    return Err(IsolationError::Crashed(error.raw_os_error()));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn worker_failure_is_contained_and_reported() {
        let mut isolation = ProcessingIsolation::new(IsolationPolicy {
            timeout: Duration::from_millis(20),
            latency_frames: 128,
        });
        let result = isolation.run_worker("sh", &["-c", "exit 7"]);
        assert!(matches!(result, Err(IsolationError::Crashed(Some(7)))));
        assert_eq!(isolation.state(), IsolationState::Crashed);
        assert_eq!(isolation.latency_frames(), 128);
    }

    #[cfg(unix)]
    #[test]
    fn worker_timeout_is_contained() {
        let mut isolation = ProcessingIsolation::new(IsolationPolicy {
            timeout: Duration::from_millis(5),
            latency_frames: 64,
        });
        let result = isolation.run_worker("sh", &["-c", "sleep 1"]);
        assert!(matches!(result, Err(IsolationError::TimedOut)));
        assert_eq!(isolation.state(), IsolationState::TimedOut);
    }
}
