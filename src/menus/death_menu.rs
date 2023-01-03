use crate::resources::*;
use bevy::prelude::*;
pub struct DeathMenuPlugin;

pub fn setup_death_menu(mut commands: Commands, asset_cache: Res<MenuAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                size: Size::new(Val::Percent(90.0), Val::Percent(80.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.0), Val::Percent(100.0)),
                    margin: UiRect::horizontal(Val::Percent(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: asset_cache.button.clone().into(),
                ..default()
            });
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.0), Val::Percent(100.0)),
                    margin: UiRect::horizontal(Val::Percent(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: asset_cache.button.clone().into(),
                ..default()
            });
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.0), Val::Percent(100.0)),
                    margin: UiRect::horizontal(Val::Percent(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: asset_cache.button.clone().into(),
                ..default()
            });
        });
}
