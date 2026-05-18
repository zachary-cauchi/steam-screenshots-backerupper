use serde_json::Value;
use tracing::{error, info, warn};
use ureq::http::StatusCode;

use crate::{
    result::{CrateError, CrateResult},
    steam::Steam,
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

        for screenshots_dir in screenshots_dirs.iter().rev() {
            info!("Game dir: {:?}", screenshots_dir);

            let Some(game_id) = screenshots_dir.file_name() else {
                warn!("Path has no name. Path: {}", screenshots_dir.display());
                continue;
            };
            let game_id = game_id.to_string_lossy();

            info!("Fetching name of game '{}'.", game_id);

            let Some(game_name) = self.get_name_from_steamstore(game_id.as_ref())? else {
                warn!("No name for Game ID '{game_id}' found.");
                continue;
            };

            info!("Game name: {game_name}");

            self.u2c.upload(&screenshots_dir, &game_name)?;

            info!("Screenshots for '{game_name}' uploaded.");
            break;
        }

        Ok(())
    }

    /// Gets the game name using the Steam webstore API.
    fn get_name_from_steamstore(&self, game_id: &str) -> CrateResult<Option<String>> {
        const STEAM_STORE_API_URL: &str = "https://store.steampowered.com/api/appdetails";

        let url = format!("{STEAM_STORE_API_URL}?appids={}", game_id.to_string());
        info!("Calling '{url}'.");

        let mut response = ureq::get(url).call()?;

        if response.status() != StatusCode::OK {
            warn!(
                "Non-OK status. API call returned status '{}'",
                response.status()
            );
        }

        let game_json: Value = response.body_mut().read_json()?;

        if game_json[game_id]["success"] != Value::Bool(true) {
            return Ok(None);
        }

        let name = match &game_json[game_id]["data"]["name"] {
            Value::String(name) => name.clone(),
            Value::Null => return Err(CrateError::DataNotFound(format!("{game_id}.data.name"))),
            _ => return Err(CrateError::WrongType(("String", "Non-string type"))),
        };

        Ok(Some(name))
    }
}
