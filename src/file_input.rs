use dioxus::prelude::*;
use dioxus::events::{MouseEvent, FormEvent, PointerData};


#[derive(Props)]
pub struct FileInputProps<'a> {
    file_types: &'a str,
    id: &'a str,
    oninput: EventHandler<'a, FormEvent>
}

#[allow(non_snake_case)]
pub fn FileInput<'a>(cx: Scope<'a, FileInputProps<'a>>) -> Element {
    cx.render(rsx!{
        div {
            class: "reveal-on-img-mouseover",
            label {
                r#for: format_args!("{}", cx.props.id),
                class: "button button-outline",
                width: "10rem",
                "Select Image"
            }
            input {
                style: "display: none",
                r#type: "file",
                id: format_args!("{}", cx.props.id),
                name: format_args!("{}", cx.props.id),
                accept: format_args!("{}", cx.props.file_types),
                oninput: move |evt| cx.props.oninput.call(evt),
                "Image"
            }
        }
    })
}