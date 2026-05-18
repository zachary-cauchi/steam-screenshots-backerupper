use serde_json::Value;
use tracing::{debug, instrument, warn};
use ureq::http::StatusCode;

use crate::result::{CrateError, CrateResult};

pub struct GameNameGetter {}

impl GameNameGetter {
    const STEAM_STORE_API_URL: &str = "https://store.steampowered.com/api/appdetails";

    pub fn new() -> Self {
        GameNameGetter {}
    }

    /// Gets the game name using the Steam webstore API.
    #[instrument(skip(self), ret(level = "DEBUG"), err(level = "DEBUG"))]
    pub fn game_id_to_name(&self, game_id: &str) -> CrateResult<Option<String>> {
        let url = format!(
            "{}?appids={}",
            Self::STEAM_STORE_API_URL,
            game_id.to_string()
        );

        debug!("Calling '{url}'.");

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
