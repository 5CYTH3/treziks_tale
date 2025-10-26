pub mod player;
pub mod ui;
use bevy::app::{App, Plugin};

use self::{player::PlayerPlugin, ui::UiPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin).add_plugins(UiPlugin);
    }
}
