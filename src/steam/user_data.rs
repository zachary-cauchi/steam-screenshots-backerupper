use std::path::{Path, PathBuf};

use tracing::{error, warn};

use crate::result::{CrateError, CrateResult};

#[derive(Debug)]
pub struct UserData {
    pub user_id: u64,
    path: PathBuf,
}

impl UserData {
    const SCREENSHOTS_SUBDIR: &'static str = "760/remote";

    pub(super) fn new(path: PathBuf) -> CrateResult<Self> {
        let user_id = path
            .file_name()
            .ok_or_else(|| CrateError::FilePathing("path ends in .."))?
            .to_str()
            .ok_or_else(|| CrateError::FilePathing("Invalid UTF-8 in path"))?
            .parse::<u64>()?;

        Ok(Self { path, user_id })
    }

    pub fn root(&self) -> &Path {
        &self.path
    }

    fn screenshots_dir(&self) -> PathBuf {
        self.path.join(Self::SCREENSHOTS_SUBDIR)
    }

    /// Get the per-game subdirs of the user's screenshots directory.
    pub fn iter_screenshots_dir(&self) -> CrateResult<impl Iterator<Item = PathBuf>> {
        let game_dirs = self
            .screenshots_dir()
            .read_dir()
            .inspect_err(|e| {
                error!(
                    "Could not read screenshots dir for user {}. Error: {e}",
                    self.user_id
                )
            })?
            .filter_map(|result| match result {
                Ok(d) if d.file_name().to_string_lossy().parse::<u64>().is_ok() => Some(d.path()),
                Ok(_) => None,
                Err(e) => {
                    warn!("Skipping a game. Error: {e}");
                    return None;
                }
            });

        Ok(game_dirs)
    }
}
