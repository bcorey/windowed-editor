use dioxus::{prelude::*, events::FormEvent};
use crate::file_input::*;
use crate::image_mgmt::*;
use web_sys::Url;

#[allow(non_snake_case)]


#[derive(Props)]
pub struct ImagePanelProps<'a> {
    id: &'a str,
}

pub fn ImagePanel<'a>(cx: Scope<'a, ImagePanelProps<'a>>) -> Element {

    let image_source = use_state(&cx, || "background-image: url(DSC00318.jpg)".to_string());

    let input_event = move |_evt: FormEvent| {
        log::info!("oninput");
        if let Some(file) = get_file(&cx.props.id) {
            let url = Url::create_object_url_with_blob(file.as_ref()).unwrap();
            image_source.set(format!("background-image: url({})", url));
            log::info!("file loaded");
            /*cx.spawn(
                async move {
                    crate::simple_render_pipeline::run().await;
                }
            )*/
        }

    };

    cx.render(rsx!{
        div {
            class: "draggable-body resizable",
            div {
                style: "background-color: var(--bg); height: 100%; width: 100%;",
                div {
                    class: "img-body",
                    style: "{image_source}",
                    FileInput {
                        file_types: "jpeg",
                        id: cx.props.id,
                        oninput: input_event,
                    }
                }
            }
        }
        
        
    })
}