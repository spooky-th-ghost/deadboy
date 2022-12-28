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

fn main() {
    let mut spawn_timer = Timer::from_seconds(2.0, TimerMode::Repeating);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(YureiPluginWithState(AppState::Gameplay))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(EnemyStats::default())
        .add_state(AppState::Gameplay)
        .add_startup_system(setup_world)
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
                .with_system(handle_player_movement_input),
        )
        .run();
}
