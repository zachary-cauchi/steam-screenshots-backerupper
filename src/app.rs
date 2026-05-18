use tracing::{error, info, warn};

use crate::{
    result::CrateResult,
    steam::{name_provider::GameNameGetter, Steam},
    u2c::U2c,
};

const _JSNLI_GAME_APPID_LIST_URL: &str =
    "https://github.com/jsnli/steamappidlist/blob/master/data/games_appid.json";

pub struct App {
    u2c: U2c,
}

impl App {
    pub fn new(server_url: impl ToString, password: impl ToString) -> Self {
        Self {
            u2c: U2c::new(server_url, password),
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
        let name_getter = GameNameGetter::new();

        for screenshots_dir in screenshots_dirs.iter().rev() {
            info!("Game dir: {:?}", screenshots_dir);

            let Some(game_id) = screenshots_dir.file_name() else {
                warn!("Path has no name. Path: {}", screenshots_dir.display());
                continue;
            };
            let game_id = game_id.to_string_lossy();

            info!("Fetching name of game '{}'.", game_id);

            let Some(game_name) = name_getter.game_id_to_name(game_id.as_ref())? else {
                warn!("No name for Game ID '{game_id}' found.");
                continue;
            };

            info!("Retrieved name. Uploading screenshots.");

            self.u2c.upload(&screenshots_dir, &game_name)?;

            info!("Screenshots for '{game_name}' uploaded.");
            break;
        }

        Ok(())
    }
}
