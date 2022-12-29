use crate::traits::*;
use bevy::prelude::*;

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
    pub hitstun_timer: Timer,
    pub in_hitstun: bool,
}

impl Default for EnemyHealth {
    fn default() -> Self {
        EnemyHealth {
            health: 10,
            hitstun_timer: Timer::from_seconds(0.33, TimerMode::Once),
            in_hitstun: false,
        }
    }
}
