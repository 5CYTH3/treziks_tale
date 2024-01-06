use bevy::{app::{Plugin, App}, ecs::schedule::OnEnter};

use crate::AppState;

mod layout;
mod styles;
mod components;
mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), layout::spawn_health_bar);
    }
}