use std::path::{Path, PathBuf};

use tracing::{debug, error, warn};
use ureq::Agent;

use crate::result::{CrateError, CrateResult};

#[derive(Debug)]
pub struct UserData {
    user_id: u64,
    path: PathBuf,
}

impl UserData {
    fn new(path: PathBuf) -> CrateResult<Self> {
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
        self.path.join("760/remote")
    }

    /// Get the per-game subdirs of the user's screenshots directory.
    fn iter_screenshots_dir(&self) -> CrateResult<Vec<PathBuf>> {
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
            })
            .collect::<Vec<_>>();

        Ok(game_dirs)
    }
}

#[derive(Debug)]
pub struct Steam {
    root: PathBuf,
    client: Agent,
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

        let client = ureq::agent();

        Ok(Self {
            root: steam_dir,
            client,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn get_game_screenshot_dirs(&self) -> CrateResult<Vec<(UserData, Vec<PathBuf>)>> {
        let screenshots_dirs = self
            .get_users()?
            .into_iter()
            .map(|user| match user.iter_screenshots_dir() {
                Ok(dirs) => (user, dirs),
                Err(e) => {
                    warn!(
                        "Failed to collect screenshot dirs for user '{}'. Error: {e}",
                        user.user_id
                    );
                    (user, vec![])
                }
            })
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
