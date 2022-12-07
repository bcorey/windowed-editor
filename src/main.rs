
use dioxus::prelude::*;
use wasm_logger;
use dioxus::events::{PointerData, onpointerdown };
use dioxus::core::UiEvent;
use std::collections::HashMap;
use dioxus::fermi::*;

mod draggable;
mod drag_container;
mod drag_content;
mod image_panel;
mod file_input;
mod image_mgmt;
mod ui_panel;
mod text_panel;

use drag_container::*;
use drag_content::*;
use image_panel::*;
use ui_panel::*;
use text_panel::*;

use web_sys::CssStyleDeclaration;
use wasm_bindgen::JsCast;

#[derive(Debug, PartialEq, Clone, Copy)]
struct ColorEntry<'a> {
    accent: &'a str,
    fg: &'a str,
    bg: &'a str,
    blendmode: &'a str,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

fn get_style_obj() -> CssStyleDeclaration {
    web_sys::window()
    .unwrap().document()
    .unwrap().document_element()
    .unwrap().dyn_into::<web_sys::HtmlElement>().map_err(|_| ())
    .unwrap().style()
}

fn update_styles(style_obj: CssStyleDeclaration, active_color: ColorEntry) {
    style_obj.set_property("--accent", active_color.accent).expect("failed to set accent");
    style_obj.set_property("--active", active_color.accent).expect("failed to set accent");

    style_obj.set_property("--fg", active_color.fg).expect("failed to set fg");
    style_obj.set_property("--bg", active_color.bg).expect("failed to set bg");
    style_obj.set_property("--mg", active_color.fg).expect("failed to set accent");
    style_obj.set_property("--blend-mode", active_color.blendmode).expect("failed to set blendmode");

}

fn app(cx: Scope) -> Element {
    
    log::info!("page update");

    let mut style_list: Vec<ColorEntry> = Vec::new();
    style_list.push(ColorEntry {
        accent: "#46FF5D",
        bg: "#250EAE",
        fg: "#FFFFFF",
        blendmode: "screen"
    });
    style_list.push(ColorEntry {
        accent: "#00FF00",
        bg: "#000000",
        fg: "#FFFFFF",
        blendmode: "screen"
    });
    style_list.push(ColorEntry {
        accent: "#fdefe9",
        bg: "#004651",
        fg: "#FFFFFF",
        blendmode: "screen"
    });
    style_list.push(ColorEntry {
        accent: "#3210f1",
        bg: "#FFFFFF",
        fg: "#000000",
        blendmode: "darken"

    });

    let current_style = use_state(&cx, || 0);


    let next_style_action = move |_event: UiEvent<PointerData>| {
        if **current_style < style_list.len() - 1 {
            current_style.set(**current_style + 1);
        } else {
            current_style.set(0);
        }
        let style_obj = get_style_obj(); // innefficient - retrieves style object via web_sys on each run
        update_styles(style_obj, *style_list.get(**current_style).unwrap());
    };


    let active_record: &UseAtomRef<HashMap<String, bool>> = use_atom_ref(&cx, ACTIVE_RECORD);

    let open_panel_action = move |_event: UiEvent<PointerData>| {
        log::info!("open panel pressed");
        active_record.write().iter_mut().filter(|(k, v)| !**v && k.contains("image"))
            .next()
            .map(|(_k, v)| {
                *v = true;
                log::info!("reached map");
        });
    };

    let open_text_panel_action = move |_event: UiEvent<PointerData>| {
        log::info!("open panel pressed");
        active_record.write().iter_mut().filter(|(k, v)| !**v && k.contains("text"))
            .next()
            .map(|(_k, v)| {
                *v = true;
                log::info!("reached map");
        });
    };

    let mut img_panels: Vec<DragContent> = Vec::new();

    for item in 0..2 {
        img_panels.push(DragContent::new_extra(
            cx.render(rsx!{
                ImagePanel {
                    key: "{item}",
                    id: "{item}",
                }
            }),
            format!("image {}", item),
        ));
    }

    for item in 3..10 {
        img_panels.push(DragContent::new_hidden(
            cx.render(rsx!{
                ImagePanel {
                    key: "{item}",
                    id: "{item}",
                }
            }),
            format!("image {}", item),
        ));
    }

    for item in 11..15 {
        img_panels.push(DragContent::new_hidden(
            cx.render(rsx!{
                TextPanel {
                    key: "{item}",
                }
            }),
            format!("text {}", item),
        ));
    }

    img_panels.push(DragContent::new(
        cx.render(rsx!{
            UIPanel {
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: open_panel_action,
                    "add image"
                }
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: open_text_panel_action,
                    "add text"
                }
                div { class: "br" }

            }
        }),
        "Add Items".to_string(),
        false
    ));

    img_panels.push(DragContent::new(
        cx.render(rsx!{
            UIPanel {
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: open_panel_action,
                    "flip image"
                }
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: open_text_panel_action,
                    "invert"
                }
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: open_panel_action,
                    "brighten"
                }
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: open_text_panel_action,
                    "darken"
                }
                div { class: "br" }
            }
        }),
        "transform".to_string(),
        false
    ));

    img_panels.push(DragContent::new(
        cx.render(rsx!{
            UIPanel {
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: next_style_action,
                    "next"
                }
                div { class: "br" }
                button {
                    class: "button button-outline list-button",
                    onpointerdown: open_text_panel_action,
                    "previous"
                }
                div { class: "br" }

            }
        }),
        "theme".to_string(),
        false
    ));
    
    /*better_content.push(DragContent::new(
        cx.render(rsx!{
            button {
                onpointerdown: move |evt| {
                    add_image_action(evt);
                    log::info!("button clicked");
                },
                "add image"
            }
        }),
        "content builder".to_string(),
        false
    ));
    better_content.push(DragContent::new(
        cx.render(rsx!{
            h3 {
                "did it"
            }
        }),
        "V2".to_string(),
        true,
    ));
    contents.write().append(&mut better_content);*/
   

    cx.render(rsx!{
        DragArea {
            content: img_panels,
        }
        div {

        }
    })
}

