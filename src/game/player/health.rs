use bevy::ecs::component::Component;


#[derive(Component)]
pub struct Health(pub i8);

pub const MAX_HEALTH: i8 = 12;
pub const HEART_COUNT: i8 = 3;

impl Health {
    pub fn to_hearts(&self) -> (i8, i8) {
        let hearts = self.0 / HEART_COUNT;
        let heart_pieces = self.0 % 4;
        (hearts, heart_pieces)
    }
}