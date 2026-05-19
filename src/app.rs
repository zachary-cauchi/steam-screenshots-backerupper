use std::{ffi::OsStr, path::Path};

use tracing::{error, info, instrument, warn};

use crate::{
    result::{CrateError, CrateResult},
    steam::{name_provider::GameNameGetter, Steam},
    u2c::U2c,
};

const _JSNLI_GAME_APPID_LIST_URL: &str =
    "https://github.com/jsnli/steamappidlist/blob/master/data/games_appid.json";

pub struct App {
    u2c: U2c,
    name_getter: GameNameGetter,
}

impl App {
    pub fn new(server_url: impl ToString, password: impl ToString) -> Self {
        Self {
            u2c: U2c::new(server_url, password),
            name_getter: GameNameGetter::new(),
        }
    }

    /// Main runtime.
    pub fn run(&self) -> CrateResult<()> {
        info!("App started");

        info!("Getting Steam directory.");
        let steam_dir = Steam::from_users_home_dir()
            .inspect_err(|e| error!("Failed to locate steam dir. Error: {e}"))?;

        info!("Steam dir: {:?}", steam_dir.root());

        info!("Getting screenshots directory.");
        let screenshots_dirs = steam_dir.get_game_screenshot_dirs()?;

        for screenshots_dir in screenshots_dirs.iter() {
            if let Err(e) = self.process_screenshots(screenshots_dir) {
                warn!(
                    "Failed to process directory '{}'. Error: {e}",
                    screenshots_dir.display()
                );
            }
        }

        Ok(())
    }

    /// Process the game screenshots in a directory.
    /// This involves resolving the name of the game in question and then uploading them.
    #[instrument(skip_all)]
    fn process_screenshots(&self, screenshots_dir: &Path) -> CrateResult<()> {
        info!("Game dir: {:?}", screenshots_dir);

        let game_id = screenshots_dir
            .file_name()
            .map(OsStr::to_string_lossy)
            .ok_or(CrateError::FilePathing("Path has no filename"))?;

        info!("Fetching name of game '{}'.", game_id);

        let game_name =
            self.name_getter
                .game_id_to_name(game_id.as_ref())?
                .ok_or(CrateError::DataNotFound(format!(
                    "No name for Game ID '{game_id}'"
                )))?;

        info!("Retrieved name. Uploading screenshots.");

        self.u2c.upload(&screenshots_dir, &game_name)?;

        info!("Screenshots for '{game_name}' uploaded.");
        Ok(())
    }
}
