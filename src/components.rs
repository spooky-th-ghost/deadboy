use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum Pickup {
    Experience(u16),
    Health(u16),
    Essence(u16),
}
