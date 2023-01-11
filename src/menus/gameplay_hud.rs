use crate::{AppState, KBundle, PlayerHealth};
use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Component, Clone, PartialEq)]
pub struct GameplayHudProps {
    app_state: AppState,
}

impl Default for GameplayHudProps {
    fn default() -> Self {
        Self {
            app_state: AppState::Gameplay,
        }
    }
}

impl Widget for GameplayHudProps {}

KBundle!(GameplayHudBundle, GameplayHudProps);

pub fn gameplay_hud_render(
    In((mut widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    mut query: Query<&KChildren>,
    player_health: Res<PlayerHealth>,
) -> bool {
    let parent_id = Some(entity);

    let health_percentage = player_health.current_health as f32 / player_health.max_health as f32;
    if let Ok(children) = query.get(entity) {
        let background_styles = KStyle {
            layout_type: StyleProp::Value(LayoutType::Row),
            background_color: StyleProp::Value(Color::RED),
            border_radius: Corner::all(50.0).into(),
            width: StyleProp::Value(Units::Pixels(500.0 * health_percentage)),
            height: StyleProp::Value(Units::Pixels(50.0)),
            ..Default::default()
        };

        rsx! {
            <BackgroundBundle
                styles={background_styles}
                children={children.clone()}
            />
        };
    }

    true
}

pub fn setup_gameplay_hud(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;

    widget_context.add_widget_data::<GameplayHudProps, EmptyState>();

    widget_context.add_widget_system(
        GameplayHudProps::default().get_name(),
        widget_update::<GameplayHudProps, EmptyState>,
        gameplay_hud_render,
    );

    rsx! {
        <KayakAppBundle>
            <GameplayHudBundle>
            </GameplayHudBundle>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
