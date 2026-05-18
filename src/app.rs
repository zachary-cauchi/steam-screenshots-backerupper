use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    thread::spawn,
};

use serde_json::Value;
use tracing::{error, info, warn};
use ureq::http::{uri::Scheme, StatusCode, Uri};

use crate::{
    result::{CrateError, CrateResult},
    steam::Steam,
    uploader::Uploader,
};

const JSNLI_GAME_APPID_LIST_URL: &str =
    "https://github.com/jsnli/steamappidlist/blob/master/data/games_appid.json";

pub struct App {
    pub server_url: String,
    pub pass: String,
}

impl App {
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

            self.upload(&screenshots_dir, &game_name)?;

            info!("Screenshots for '{game_name}' uploaded.");
            break;
        }

        Ok(())
    }

    /// Upload all files in the given dir to the server.
    pub fn upload(&self, screenshots_dir: &Path, game_name: &str) -> CrateResult<()> {
        /// The destination, composed of the server URL, base screenshots path, and game name as the final directory.
        /// Since some games can have certain characters invalid for URLs (such as colons), encode the name.
        let destination = format!("{}/{}", self.server_url, urlencoding::encode(game_name));

        let (mut reader, writer) = std::io::pipe()?;
        let mut cmd = Command::new("tool_u2c");
        cmd.arg("-a")
            .arg(&self.pass)
            .arg(destination.to_string())
            .arg(screenshots_dir.join("screenshots"))
            .stdout(writer.try_clone()?)
            .stderr(writer);

        info!(
            "Running cmd '{:?}' with args {:?}",
            cmd.get_program(),
            cmd.get_args().collect::<Vec<_>>()
        );

        let mut spawned = cmd.spawn()?;

        let buf_reader = BufReader::new(reader);
        for line in buf_reader.lines() {
            info!(
                "u2c out: {}",
                line.as_ref().map_or("LINE_ERR", String::as_str)
            );
        }

        let status = spawned.wait_with_output()?.status;
        info!("u2c exited with status '{}'", status);
        // info!("stdout:\n\t{}", String::from_utf8_lossy(&output.stdout));
        // info!("stderr:\n\t{}", String::from_utf8_lossy(&output.stderr));

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
            v => return Err(CrateError::WrongType(("String", "Non-string type"))),
        };

        Ok(Some(name))
    }
}
