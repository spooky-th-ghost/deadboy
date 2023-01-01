use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
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
    let mut spawn_timer = Timer::from_seconds(2.0, TimerMode::Repeating);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(YureiPluginWithState(AppState::Gameplay))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(EnemyStats::default())
        .insert_resource(WeaponEntities::default())
        .insert_resource(PlayerInventory::default())
        .insert_resource(PlayerGroundPosition::default())
        .insert_resource(PlayerHealth { health: 100 })
        .add_state(AppState::Gameplay)
        .add_startup_system(setup_world)
        .add_startup_system(load_menu_assets)
        .add_startup_system(setup_menu.after(load_menu_assets))
        .add_system_set(
            SystemSet::on_update(AppState::Gameplay)
                .with_system(
                    move |cmd: Commands,
                          time: Res<Time>,
                          enemy_stats: ResMut<EnemyStats>,
                          meshes: ResMut<Assets<Mesh>>,
                          materials: ResMut<Assets<StandardMaterial>>| {
                        spawn_enemy(cmd, enemy_stats, meshes, materials, time, &mut spawn_timer)
                    },
                )
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
        .run();
}
