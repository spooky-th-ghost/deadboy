use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyStats {
    enemy_count: u8,
    max_enemy_count: u8,
}

impl EnemyStats {
    pub fn add_enemy(&mut self) {
        self.enemy_count += 1;
    }

    pub fn can_spawn_enemy(&self) -> bool {
        self.enemy_count < self.max_enemy_count
    }
}

impl Default for EnemyStats {
    fn default() -> Self {
        EnemyStats {
            enemy_count: 0,
            max_enemy_count: 255,
        }
    }
}

pub struct PlayerInventory {
    pub items: Vec<Item>,
    pub possible_items: Vec<ItemType>,
}

impl PlayerInventory {
    fn weapon_count(&self) -> usize {
        self.items
            .iter()
            .filter(|x| x.category == ItemCategory::Weapon)
            .collect::<Vec<&Item>>()
            .len()
    }

    fn support_count(&self) -> usize {
        self.items
            .iter()
            .filter(|x| x.category == ItemCategory::Support)
            .collect::<Vec<&Item>>()
            .len()
    }

    fn item_position_in_inventory(&self, item_type: ItemType) -> Option<usize> {
        self.items.iter().position(|x| x.item_type == item_type)
    }

    pub fn add_item(&mut self, item_type: ItemType) {
        // Check if we have this item in our inventory or not
        if let Some(index) = self.item_position_in_inventory(item_type) {
            // If we do, level it up
            self.items[index].level += 1;
        } else {
            // If we don't have the item, we have to make sure we can add it to our inventory
            let category = ItemCategory::from_item_type(item_type);

            if category == ItemCategory::Weapon {
                self.items.push(Item::new(item_type));
            }

            if category == ItemCategory::Support {
                self.items.push(Item::new(item_type));
            }
        }
        self.update_choices();
    }

    fn update_choices(&mut self) {
        let mut temp_possible_items = self.possible_items.clone();
        // If we have the maximum number of support items, remove all support items from the pool
        if self.support_count() >= 5 {
            temp_possible_items = temp_possible_items
                .into_iter()
                .filter(|current_item| {
                    ItemType::SUPPORT_ITEMS
                        .into_iter()
                        .any(|y| current_item == &y)
                        == false
                })
                .collect::<Vec<ItemType>>();
        }

        // If we have the maximum number of weapon items, remove all weapon items from the pool
        if self.weapon_count() >= 3 {
            temp_possible_items = temp_possible_items
                .into_iter()
                .filter(|current_item| {
                    ItemType::WEAPON_ITEMS
                        .into_iter()
                        .any(|y| current_item == &y)
                        == false
                })
                .collect::<Vec<ItemType>>();
        }

        //TODO: Add logic for removing an item from the pool when it's max level has been reached

        self.possible_items = temp_possible_items;
    }
}

pub struct Item {
    pub level: u8,
    pub item_type: ItemType,
    pub category: ItemCategory,
}

impl Item {
    pub fn new(item_type: ItemType) -> Self {
        let category = ItemCategory::from_item_type(item_type);
        Item {
            level: 0,
            item_type,
            category,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ItemType {
    // Weapons
    Halo,
    Scythe,
    Lantern,
    // Support
    Cloak,
    Pocketwatch,
    Boots,
    Monocle,
}

impl ItemType {
    const SUPPORT_ITEMS: [ItemType; 4] = [
        ItemType::Cloak,
        ItemType::Pocketwatch,
        ItemType::Boots,
        ItemType::Monocle,
    ];

    const WEAPON_ITEMS: [ItemType; 3] = [ItemType::Halo, ItemType::Scythe, ItemType::Lantern];
}
#[derive(PartialEq)]
pub enum ItemCategory {
    Weapon,
    Support,
}

impl ItemCategory {
    pub fn from_item_type(item_type: ItemType) -> Self {
        use ItemType::*;
        match item_type {
            Halo | Scythe | Lantern => ItemCategory::Weapon,
            _ => ItemCategory::Support,
        }
    }
}
