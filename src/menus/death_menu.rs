use crate::{AppState, KBundle};
use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Component, Clone, PartialEq)]
pub struct MyButtonProps {
    app_state: AppState,
}

impl Default for MyButtonProps {
    fn default() -> Self {
        Self {
            app_state: AppState::Gameplay,
        }
    }
}

impl Widget for MyButtonProps {}

KBundle!(MyButtonBundle, MyButtonProps);

pub fn my_button_render(
    In((mut widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    mut query: Query<&KChildren>,
) -> bool {
    let parent_id = Some(entity);

    if let Ok(children) = query.get(entity) {
        let background_styles = KStyle {
            layout_type: StyleProp::Value(LayoutType::Row),
            background_color: StyleProp::Value(Color::RED),
            border_radius: Corner::all(50.0).into(),
            width: StyleProp::Value(Units::Pixels(500.0)),
            height: StyleProp::Value(Units::Pixels(500.0)),
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

pub fn setup_death_menu(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;

    widget_context.add_widget_data::<MyButtonProps, EmptyState>();

    widget_context.add_widget_system(
        MyButtonProps::default().get_name(),
        widget_update::<MyButtonProps, EmptyState>,
        my_button_render,
    );

    rsx! {
        <KayakAppBundle>
            <MyButtonBundle>
                <TextWidgetBundle
                    text={TextProps {
                        content: "First Option".into(),
                        size: 40.0,
                        ..Default::default()
                    }}

                />
                <TextWidgetBundle
                    text={TextProps {
                        content: "Second Option".into(),
                        size: 40.0,
                        ..Default::default()
                    }}

                />
                <TextWidgetBundle
                    text={TextProps {
                        content: "Thrid Option".into(),
                        size: 40.0,
                        ..Default::default()
                    }}

                />
            </MyButtonBundle>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
