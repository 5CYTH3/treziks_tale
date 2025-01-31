use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::{OnEnter, IntoSystemConfigs};
use bevy::ecs::system::Res;
use bevy::prelude::{Transform, Vec2, KeyCode, Bundle, GamepadButtonType, App, Plugin, Update};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::{prelude::{Component, With, Commands, Query, AssetServer}, window::{Window, PrimaryWindow}};
use leafwing_input_manager::InputManagerBundle;
use leafwing_input_manager::prelude::{InputMap, InputManagerPlugin};

pub mod actions;
pub mod inventory;
pub mod health;

use actions::PlayerActions;

use crate::AppState;

use self::health::Health;
use self::inventory::InventoryPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerActions>::default())
            .add_plugins(InventoryPlugin)
            .add_systems(Update, actions::player_movements.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    input_manager: InputManagerBundle<PlayerActions>,
    sprite: SpriteBundle,
    health: Health
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerActions> {
        let mut input_map = InputMap::default();

        use PlayerActions::*;
        // Movement
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        // Abilities
        input_map.insert(KeyCode::J, Attack);
        input_map.insert(GamepadButtonType::South, Attack);

        input_map.insert(KeyCode::K, SecondaryAttack);
        input_map.insert(GamepadButtonType::West, SecondaryAttack);

        input_map.insert(KeyCode::E, HeavyAttack);
        input_map.insert(GamepadButtonType::East, HeavyAttack);

        input_map
    }
}

pub fn spawn_player(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = query.get_single().unwrap();

    commands.spawn(PlayerBundle {
        player: Player,
        input_manager: InputManagerBundle { input_map: PlayerBundle::default_input_map(), ..Default::default() },
        sprite: SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("HeroIdleFrontSprite.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        health: Health(12)
    });
}