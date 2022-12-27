use crate::resources::ItemType;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
//#[reflect(Component)]
pub struct ItemPickup {
    pub item_options: Vec<ItemType>,
}

impl ItemPickup {
    pub fn new(possible_items: &Vec<ItemType>) -> Self {
        let mut temp_vec = possible_items.clone();
        let mut item_options: Vec<ItemType> = Vec::new();

        let max_choices = std::cmp::min(possible_items.len(), 3);

        while item_options.len() < max_choices {
            let index = rand::thread_rng().gen_range(0..temp_vec.len());
            item_options.push(temp_vec[index]);
            temp_vec.remove(index);
        }

        ItemPickup { item_options }
    }
}
