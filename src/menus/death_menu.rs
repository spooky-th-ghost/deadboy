use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

pub fn setup_death_menu(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    let parent_id = None;

    // The rsx! macro expects a parent_id, a widget_context from the user.
    // It also expects `Commands` from bevy.
    // This can be a little weird at first.
    // See the rsx! docs for more info!
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
// pub fn old_setup_death_menu(mut commands: Commands, asset_cache: Res<MenuAssets>) {
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 align_self: AlignSelf::Center,
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::SpaceBetween,
//                 size: Size::new(Val::Percent(90.0), Val::Percent(80.0)),
//                 margin: UiRect::all(Val::Auto),
//                 ..default()
//             },
//             background_color: BackgroundColor(Color::NONE),
//             ..default()
//         })
//         .with_children(|parent| {
//             parent.spawn(ImageBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(90.0), Val::Percent(100.0)),
//                     margin: UiRect::horizontal(Val::Percent(4.0)),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 image: asset_cache.button.clone().into(),
//                 ..default()
//             });
//             parent.spawn(ImageBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(90.0), Val::Percent(100.0)),
//                     margin: UiRect::horizontal(Val::Percent(4.0)),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 image: asset_cache.button.clone().into(),
//                 ..default()
//             });
//             parent.spawn(ImageBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(90.0), Val::Percent(100.0)),
//                     margin: UiRect::horizontal(Val::Percent(4.0)),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 image: asset_cache.button.clone().into(),
//                 ..default()
//             });
//         });
// }
