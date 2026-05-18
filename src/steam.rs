pub mod user_data;

use std::path::{Path, PathBuf};

use tracing::{debug, error, warn};

use crate::{
    result::{CrateError, CrateResult},
    steam::user_data::UserData,
};

#[derive(Debug)]
pub struct Steam {
    root: PathBuf,
}

impl Steam {
    const HOME_PATH: &str = ".steam/steam";

    /// Construct [Self] by finding the Steam installation dir via the user's home dir.
    /// This is done by looking for "~/.steam/steam" and following the sym-link to the actual install dir.
    pub fn from_users_home_dir() -> CrateResult<Self> {
        let Some(home_dir) = std::env::home_dir() else {
            warn!("Failed to find user home dir.");
            return Err(CrateError::dir_not_found("User home dir."));
        };

        debug!("Found user home dir ({}).", home_dir.display());

        debug!("Testing for `.steam/steam` dir existance.");

        let home_steam_dir = home_dir.join(Self::HOME_PATH);

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

        Ok(Self { root: steam_dir })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn get_game_screenshot_dirs(&self) -> CrateResult<Vec<PathBuf>> {
        let screenshots_dirs = self
            .get_users()?
            .into_iter()
            .filter_map(|user| {
                user.iter_screenshots_dir()
                    .inspect_err(|e| {
                        warn!(
                            "Failed to collect screenshot dirs for user '{}'. Error: {e}",
                            user.user_id
                        )
                    })
                    .ok()
            })
            .flatten()
            .collect::<Vec<_>>();

        Ok(screenshots_dirs)
    }

    /// Get the screenshots directories for each user. Users which cannot be read are filtered out.
    fn get_users(&self) -> CrateResult<Vec<UserData>> {
        debug!("Getting all screenshot subdirs.");

        let users = self
            .root
            .join("userdata")
            .read_dir()
            .inspect_err(|e| error!("Could not load Steam `userdata` dir. Error: {e}"))?
            .filter_map(|result| {
                let user_dir = result
                    .inspect_err(|e| warn!("Skipping a user in the `userdata` dir. Error: {e}"))
                    .ok()?;
                UserData::new(user_dir.path())
                    .inspect_err(|e| warn!("Failed to construct user info. Error: {e}"))
                    .ok()
            })
            .collect::<Vec<_>>();

        Ok(users)
    }
}
