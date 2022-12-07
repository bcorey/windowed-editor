use dioxus::{prelude::*, events::FormEvent};

#[allow(non_snake_case)]


#[derive(Props)]
pub struct TextPanelProps<'a> {
    children: Element<'a>
}

pub fn TextPanel<'a>(cx: Scope<'a, TextPanelProps<'a>>) -> Element {

    cx.render(rsx!{
        div {
            class: "draggable-body resizable",
            style: "background-color: transparent",
            textarea {
                spellcheck: "false",
                "hi there"
            }
            
        }
        
    })
}