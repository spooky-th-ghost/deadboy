use crate::resources::*;
use bevy::prelude::*;
pub struct DeathMenuPlugin;

pub fn setup_menu(mut commands: Commands, asset_cache: Res<AssetCache>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: asset_cache.button.clone().into(),
                ..default()
            });
        });
}
