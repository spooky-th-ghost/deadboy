use crate::traits::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct PlayerHitbox {
    pub damage: u16,
}

// Weapon Markers
#[derive(Component)]
pub struct Halo;

impl Weapon for Halo {
    fn damage_by_level(level: u8) -> WeaponDamage {
        let auto_attack_damage = Damage {
            base_damage: 4 + ((level - 1) as f32 * 0.8) as u16,
            damage_effects: Vec::new(),
        };

        let cooldown_attack_damage = Damage {
            base_damage: 15 + ((level - 1) as f32 * 5.0) as u16,
            damage_effects: Vec::new(),
        };

        WeaponDamage {
            auto_attack_damage,
            cooldown_attack_damage,
        }
    }
}

#[derive(Component)]
pub enum Pickup {
    Experience(u16),
    Health(u16),
    Essence(u16),
}

#[derive(Component)]
pub struct EnemyHealth {
    pub health: u16,
}

impl Default for EnemyHealth {
    fn default() -> Self {
        EnemyHealth { health: 10 }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct CameraController {
    pub distance: f32,
    pub angle: f32,
    pub easing: f32,
    pub target_position: Vec3,
    pub player_position: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        CameraController {
            distance: 15.0,
            angle: 0.0,
            easing: 6.0,
            target_position: Vec3::ZERO,
            player_position: Vec3::ZERO,
        }
    }
}
#[derive(Component)]
pub struct EnemyHitstun(Timer);

impl EnemyHitstun {
    pub fn new(duration: f32) -> Self {
        EnemyHitstun(Timer::from_seconds(duration, TimerMode::Once))
    }

    pub fn tick(&mut self, duration: Duration) {
        self.0.tick(duration);
    }

    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }
}
