use crate::{resources::*, AppState, HealthBarFill};
use bevy::prelude::*;

pub struct GameplayHudPlugin;

impl Plugin for GameplayHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Gameplay).with_system(spawn_gameplay_hud))
            .add_system_set(SystemSet::on_update(AppState::Gameplay).with_system(adjust_healthbar));
    }
}

pub fn spawn_gameplay_hud(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(10.0)),
                        margin: UiRect::horizontal(Val::Percent(4.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GRAY),
                    ..default()
                })
                .with_children(|s_parent| {
                    s_parent
                        .spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_self: AlignSelf::FlexStart,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::RED),
                            ..default()
                        })
                        .insert(HealthBarFill);
                });
        });
}

pub fn adjust_healthbar(
    player_health: Res<PlayerHealth>,
    time: Res<Time>,
    mut query: Query<&mut Style, With<HealthBarFill>>,
) {
    for mut style in &mut query {
        let health_percent =
            (player_health.current_health as f32 / player_health.max_health as f32) * 100.0;

        style.size.width = Val::Percent(health_percent);
    }
}
