use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemyStats {
    enemy_count: u8,
    max_enemy_count: u8,
}

#[derive(Component, Default)]
pub struct Movement {
    pub target: Option<Vec3>,
    pub current_speed: f32,
    pub top_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

impl Movement {
    fn decelerate(&mut self, delta: f32) {
        self.current_speed =
            self.current_speed + (0.0 - self.current_speed) * delta * self.deceleration;
    }

    fn accelerate(&mut self, delta: f32) {
        if self.current_speed < self.top_speed {
            self.current_speed = self.current_speed
                + (self.top_speed - self.current_speed) * delta * self.acceleration;
        }
    }

    pub fn update(&mut self, delta: f32) {
        if let Some(_) = self.target {
            self.accelerate(delta);
        } else {
            self.decelerate(delta);
        }
    }
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

fn main() {
    let mut spawn_timer = Timer::from_seconds(2.0, TimerMode::Repeating);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .insert_resource(EnemyStats::default())
        .add_startup_system(setup_world)
        .add_system(
            move |cmd: Commands,
                  time: Res<Time>,
                  enemy_stats: ResMut<EnemyStats>,
                  meshes: ResMut<Assets<Mesh>>,
                  materials: ResMut<Assets<StandardMaterial>>| {
                spawn_enemy(cmd, enemy_stats, meshes, materials, time, &mut spawn_timer)
            },
        )
        .add_system(handle_player_movement_input)
        .add_system(handle_speed)
        .add_system(handle_movement.after(handle_speed))
        .run();
}

fn setup_world(
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
        .insert(Player);
    // Spawn Player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::default(),
            ..default()
        })
        .insert(Player)
        .insert(Movement {
            top_speed: 10.0,
            acceleration: 0.75,
            deceleration: 3.5,
            ..default()
        });
}

fn spawn_enemy(
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
        let transform = Transform::from_xyz(x, 0.0, z);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::BLACK.into()),
                transform,
                ..default()
            })
            .insert(Enemy);
        enemy_stats.add_enemy();
    }
}

fn handle_speed(time: Res<Time>, mut query: Query<&mut Movement>) {
    for mut movement in &mut query {
        movement.update(time.delta_seconds());
    }
}

fn handle_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in &mut query {
        if let Some(target) = movement.target {
            let current_position = transform.translation;
            transform.look_at(current_position + target, Vec3::Y);
        }

        if movement.current_speed > 0.0 {
            let forward_vector = transform.forward().normalize();
            transform.translation += forward_vector * movement.current_speed * time.delta_seconds();
        }
    }
}

fn handle_player_movement_input(
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

        movement.target = if final_vec != Vec3::ZERO {
            Some(final_vec)
        } else {
            None
        };
    }
}
