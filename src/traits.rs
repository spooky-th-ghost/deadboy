pub struct WeaponDamage {
    pub auto_attack_damage: Damage,
    pub cooldown_attack_damage: Damage,
}

pub enum StatusEffects {
    Fire,
    Darkness,
    Ice,
}

pub struct Damage {
    pub base_damage: u16,
    pub damage_effects: Vec<StatusEffects>,
}

pub trait Weapon {
    fn damage_by_level(level: u8) -> WeaponDamage;
}
