#[macro_export]
macro_rules! KBundle {
    ($i:ident, $typ:ty) => {
        #[derive(Bundle)]
        pub struct $i {
            pub props: $typ,
            pub styles: KStyle,
            pub computed_styles: ComputedStyles,
            pub children: KChildren,
            pub on_event: OnEvent,
            pub widget_name: WidgetName,
        }

        impl Default for $i {
            fn default() -> Self {
                Self {
                    props: <$typ>::default(),
                    styles: KStyle::default(),
                    computed_styles: ComputedStyles::default(),
                    children: KChildren::default(),
                    on_event: OnEvent::default(),
                    widget_name: <$typ>::default().get_name(),
                }
            }
        }
    };
}
