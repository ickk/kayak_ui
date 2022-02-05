use crate::core::{
    layout_cache::Space,
    render_command::RenderCommand,
    rsx,
    styles::{Style, StyleProp},
    widget, Children, OnEvent, WidgetProps,
};

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct NinePatchProps {
    pub handle: u16,
    pub border: Space,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(Children)]
    pub children: Option<Children>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
    #[prop_field(Focusable)]
    pub focusable: Option<bool>,
}

#[widget]
pub fn NinePatch(props: NinePatchProps) {
    props.styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::NinePatch {
            handle: props.handle,
            border: props.border,
        }),
        ..props.styles.clone().unwrap_or_default()
    });

    rsx! {
        <>
            {children}
        </>
    }
}
