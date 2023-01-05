use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use kayak_ui::{prelude::*, widgets::*};
use yurei::prelude::*;

pub mod components;
pub use components::*;

pub mod resources;
pub use resources::*;

pub mod systems;
pub use systems::*;

pub mod utility;
pub use utility::*;

pub mod traits;
pub use traits::*;

pub mod menus;
pub use menus::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::Gameplay)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec!["deadboy.assets"])
                .with_collection::<MenuAssets>(),
        )
        .add_state(AppState::AssetLoading)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(YureiPluginWithState(AppState::Gameplay))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(EnemyStats::default())
        .insert_resource(WeaponEntities::default())
        .insert_resource(PlayerInventory::default())
        .insert_resource(PlayerGroundPosition::default())
        .insert_resource(EnemySpawnTimer::default())
        .insert_resource(PlayerHealth { health: 10 })
        .add_system_set(
            SystemSet::on_update(AppState::Gameplay)
                .with_system(spawn_enemy)
                .with_system(update_player_ground_position)
                .with_system(handle_player_movement_input)
                .with_system(handle_enemy_movement)
                .with_system(handle_hitstun)
                .with_system(update_camera_target_position.after(update_player_ground_position))
                .with_system(lerp_to_camera_position.after(update_camera_target_position))
                .with_system(handle_enemy_hitbox_collision.after(handle_hitstun))
                .with_system(handle_player_hitbox_collision)
                .with_system(kill_player.after(handle_player_hitbox_collision)),
        )
        .add_system_set(SystemSet::on_enter(AppState::Gameplay).with_system(setup_world))
        .add_system_set(SystemSet::on_enter(AppState::DeathMenu).with_system(setup_death_menu))
        .run();
}
