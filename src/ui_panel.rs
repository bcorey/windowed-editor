use dioxus::{prelude::*, events::FormEvent};

#[allow(non_snake_case)]


#[derive(Props)]
pub struct UIPanelProps<'a> {
    children: Element<'a>
}

pub fn UIPanel<'a>(cx: Scope<'a, UIPanelProps<'a>>) -> Element {

    cx.render(rsx!{
        div {
            class: "ui-wrapper",
            &cx.props.children
        }
    })
}