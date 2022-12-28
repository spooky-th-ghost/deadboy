use crate::{components::*, resources::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use yurei::prelude::*;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(200.0, 0.2, 200.0))),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(RigidBody::Fixed);

    // Spawn Player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::WHITE.into()),
            ..default()
        })
        .insert(YureiBundle {
            collider: Collider::capsule_y(0.5, 0.5),
            ..default()
        })
        .insert(Player);
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut enemy_stats: ResMut<EnemyStats>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    spawn_timer: &mut Timer,
) {
    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() && enemy_stats.can_spawn_enemy() {
        let mut rng = rand::thread_rng();
        let x: f32 = (rng.gen::<f32>() * 20.0) - 10.0;
        let z: f32 = (rng.gen::<f32>() * 20.0) - 10.0;

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::BLACK.into()),
                ..default()
            })
            .insert(YureiBundle {
                transform: Transform::from_xyz(x, 0.0, z),
                collider: Collider::capsule_y(0.5, 1.0),
                ..default()
            })
            .insert(Sensor)
            .insert(Enemy);
        enemy_stats.add_enemy();
    }
}

pub fn handle_player_movement_input(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Movement, With<Player>>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    let camera_transform = camera_query.single();

    for mut movement in &mut player_query {
        let mut x = 0.0;
        let mut z = 0.0;

        let mut forward = camera_transform.forward();
        forward.y = 0.0;
        forward = forward.normalize();

        let mut left = camera_transform.left();
        left.y = 0.0;
        left = left.normalize();

        if keyboard.pressed(KeyCode::W) {
            z += 1.0;
        }

        if keyboard.pressed(KeyCode::S) {
            z -= 1.0;
        }

        if keyboard.pressed(KeyCode::A) {
            x += 1.0;
        }

        if keyboard.pressed(KeyCode::D) {
            x -= 1.0;
        }

        let left_vec: Vec3 = x * left;
        let forward_vec: Vec3 = z * forward;

        let final_vec = left_vec + forward_vec;

        movement.direction = final_vec;
    }
}

pub fn handle_autoattacks(inventory: Res<PlayerInventory>, query: Query<&Transform, With<Player>>) {
    for transform in &query {}
}
