use tracing::{error, info};

use crate::{result::CrateResult, steam::Steam};

pub struct App {}

impl App {
    /// Main runtime.
    pub async fn run(&self) -> CrateResult<()> {
        info!("App started");

        info!("Getting Steam directory.");
        let steam_dir = Steam::from_users_home_dir()
            .inspect_err(|e| error!("Failed to locate steam dir. Error: {e}"))?;

        info!("Steam dir: {:?}", steam_dir.root());

        info!("Getting screenshots directory.");
        let screenshots_dirs = steam_dir.get_game_screenshot_dirs()?;

        for screenshots_dir in screenshots_dirs {
            info!("Game dir: {:?}", screenshots_dir)
        }

        Ok(())
    }
}
