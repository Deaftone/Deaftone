use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    pub r#type: Type,
}
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Default,
    Info,
    Success,
    Warning,
    Danger,
}
// Implemented from https://github.com/ctron/patternfly-yew/blob/d1e3ca4922b96b0fb109fb6852d341f5e877596f/src/alert.rs
impl Type {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Type::Default => vec![],
            Type::Info => vec!["alert-info"],
            Type::Success => vec!["alert-success"],
            Type::Warning => vec!["alert-warning"],
            Type::Danger => vec!["alert-danger"],
        }
    }

    /*   pub fn aria_label(&self) -> &'static str {
        match self {
            Type::Default => "Default alert",
            Type::Info => "Information alert",
            Type::Success => "Success alert",
            Type::Warning => "Warning alert",
            Type::Danger => "Danger alert",
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Type::Default => Icon::Bell,
            Type::Info => Icon::InfoCircle,
            Type::Success => Icon::CheckCircle,
            Type::Warning => Icon::ExclamationTriangle,
            Type::Danger => Icon::ExclamationCircle,
        }
    } */
}
#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let mut classes = Classes::from("alert");
    classes.extend(props.r#type.as_classes());
    html! {
        <div class="toast toast-end">
            <div class={classes!("alert",classes.clone())}>
                {for props.children.iter()}
            </div>
        </div>
    }
}
