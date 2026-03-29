use std::path::PathBuf;

use tracing::{debug, error, info, warn};

use crate::result::{CrateError, CrateResult};

const HOME_STEAM_DIR: &str = ".steam/steam";

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
    /// This is done by looking for "~/.steam/steam" and following the sym-link to the actual install dir.
    fn find_steam_dir(&self) -> CrateResult<PathBuf> {
        info!("Getting Steam directory.");

        let Some(home_dir) = std::env::home_dir() else {
            warn!("Failed to find user home dir.");
            return Err(CrateError::dir_not_found("User home dir."));
        };

        debug!("Found user home dir ({}).", home_dir.display());

        debug!("Testing for `.steam/steam` dir existance.");

        let home_steam_dir = home_dir.join(HOME_STEAM_DIR);

        if !(home_steam_dir.exists() && home_steam_dir.is_dir()) {
            warn!(
                "Could not find user's home steam dir ({}).",
                home_steam_dir.display()
            );

            return Err(CrateError::dir_not_found(home_steam_dir.to_string_lossy()));
        }

        debug!("Found user home steam dir ({}).", home_steam_dir.display());

        let steam_dir = home_steam_dir.canonicalize().inspect_err(|e| {
            warn!(
                "Failed to resolve home steam dir ({}). Error: {e}",
                home_steam_dir.display()
            )
        })?;

        Ok(steam_dir)
    }
}
