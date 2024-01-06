use bevy::{ecs::{system::{Commands, Res, Query}, query::With, entity::Entity}, asset::AssetServer, ui::{node_bundles::{NodeBundle, ImageBundle}, Style, Val, JustifyContent, FlexDirection, UiImage}, hierarchy::BuildChildren};

use crate::game::player::{Player, health::{Health, HEART_COUNT}};

use super::components::HealthBar;

pub fn spawn_health_bar(mut commands: Commands) {
    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }
    )
    .with_children(|parent| {
        parent.spawn(
            (
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                HealthBar {},
            )
        );
    });
}

/*
pub fn update_health_bar(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut health_bar_query: Query<Entity, With<HealthBar>>,
    health_query: Query<&Health, With<Player>> 
) {
    if let Ok(health) = health_query.get_single() {
        let (hearts, hearts_pieces) = health.to_hearts();
        let health_bar_entity = health_bar_query.get_single_mut().unwrap();

        let mut health_bar = commands.get_entity(health_bar_entity).unwrap(); 

        // Clear all hearts from the bar
        health_bar.clear_children();

        for _ in 1..hearts {
            commands.get_entity(health_bar_entity).unwrap().push_children(&[
                commands.spawn(ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("sprites/ui/health/full.png"),
                        ..Default::default()
                    },                
                    ..Default::default()
                }).id()
            ]);
        };

        let mut pieces = false;

        if hearts_pieces > 0 {
            commands.get_entity(health_bar_entity).unwrap().push_children(&[
                commands.spawn(ImageBundle {
                    image: UiImage {
                        texture: asset_server.load(format!("sprites/ui/health/{}.png", hearts_pieces)),
                        ..Default::default()
                    },                
                    ..Default::default()
                }).id()
            ]);
            pieces = true;
        };

        // Determine the number of empty hearts to display
        if HEART_COUNT - hearts > 0 {
            if pieces {
                for _ in 1..(HEART_COUNT - hearts - 1) {
                    commands.get_entity(health_bar_entity).unwrap().push_children(&[
                            commands.spawn(ImageBundle {
                                image: UiImage {
                                texture: asset_server.load("sprites/ui/health/empty.png"),
                                ..Default::default()
                            },                
                            ..Default::default()
                        }).id()
                    ]);
                }
            } else {
                for _ in 1..(HEART_COUNT - hearts) {
                    commands.get_entity(health_bar_entity).unwrap().push_children(&[
                            commands.spawn(ImageBundle {
                                image: UiImage {
                                texture: asset_server.load("sprites/ui/health/empty.png"),
                                ..Default::default()
                            },                
                            ..Default::default()
                        }).id()
                    ]);
                }
            }
        }
    };
}
*/