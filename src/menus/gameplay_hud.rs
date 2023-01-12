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
    let healthbar_border = ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(35.0), Val::Percent(12.5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    };

    let healthbar_background = ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(99.5), Val::Percent(90.0)),
            margin: UiRect::horizontal(Val::Percent(4.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            flex_direction: FlexDirection::Row,
            //border: UiRect::all(Val::Px(50.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::GRAY),
        ..default()
    };

    let healthbar_fill = ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_self: AlignSelf::FlexStart,
            ..default()
        },
        background_color: BackgroundColor(Color::RED),
        ..default()
    };

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
            parent.spawn(healthbar_border).with_children(|s_parent| {
                s_parent
                    .spawn(healthbar_background)
                    .with_children(|z_parent| {
                        z_parent.spawn(healthbar_fill).insert(HealthBarFill);
                    });
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
