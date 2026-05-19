use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
};

use tracing::{debug, info, instrument, warn};

use crate::result::{CrateError, CrateResult};

pub struct U2c {
    base_url: String,
    pass: String,
}

impl U2c {
    pub fn new(base_url: impl ToString, pass: impl ToString) -> Self {
        Self {
            base_url: base_url.to_string(),
            pass: pass.to_string(),
        }
    }

    /// Upload all files in the given dir to the server.
    #[instrument(name = "u2c", skip(self, screenshots_dir), err(level = "WARN"))]
    pub fn upload(&self, screenshots_dir: &Path, game_name: &str) -> CrateResult<()> {
        // The destination, composed of the server URL, base screenshots path, and game name as the final directory.
        // Since some games can have certain characters invalid for URLs (such as colons), encode the name.
        let destination = format!("{}/{}", self.base_url, urlencoding::encode(game_name));

        let mut cmd = Command::new("tool_u2c");
        cmd.arg("-a")
            .arg(&self.pass)
            .arg(destination.to_string())
            .arg(screenshots_dir.join("screenshots/"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        info!(
            "Running cmd '{:?}' with args {:?}",
            cmd.get_program(),
            cmd.get_args().collect::<Vec<_>>()
        );

        let mut spawned = cmd.spawn()?;

        // First, stream stdout.
        let buf_reader_out = BufReader::new(spawned.stdout.take().unwrap());
        for line in buf_reader_out.lines() {
            debug!(
                "u2c out: {}",
                line.as_ref().map_or("LINE_ERR", String::as_str)
            );
        }

        debug!("stdout finished. Waiting for cmd to completely exit.");

        let status = spawned.wait()?;
        debug!("u2c exited with status '{}'", status);

        // Next, stream any stderr lines if the exit code != 0.
        if !status.success() {
            warn!("Non-zero exit status. Status code: {status:?}");

            let buf_reader_err = BufReader::new(spawned.stderr.take().unwrap());
            for line in buf_reader_err.lines() {
                debug!(
                    "u2c err: {}",
                    line.as_ref().map_or("LINE_ERR", String::as_str)
                );
            }

            Err(CrateError::general("u2c exited with {status}"))
        } else {
            Ok(())
        }
    }
}
