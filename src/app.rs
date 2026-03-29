use std::path::PathBuf;

use tracing::{debug, error, info, warn};

use crate::result::{CrateError, CrateResult};

pub struct App {}

impl App {
    /// Main runtime.
    pub async fn run(&self) -> CrateResult<()> {
        info!("App started");

        let steam_dir = self
            .find_steam_dir()
            .inspect_err(|e| error!("Failed to locate steam dir. Error: {e}"))?;

        info!("Steam dir: {}", steam_dir.display());

        Ok(())
    }

    /// Find the Steam installation dir.
    fn find_steam_dir(&self) -> CrateResult<PathBuf> {
        info!("Getting Steam directory.");

        let Some(home_dir) = std::env::home_dir() else {
            warn!("Failed to find user home dir.");
            return Err(CrateError::DirNotFound("User home dir."));
        };

        debug!("Found user home dir ({}).", home_dir.display());

        Ok(home_dir)
    }
}
