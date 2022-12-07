use std::fmt::Pointer;

use dioxus::prelude::*;
use dioxus::events::{PointerData, onpointerdown};
use dioxus::core::UiEvent;
use std::collections::HashMap;
use dioxus::fermi::*;
use crate::drag_content::*;

#[derive(PartialEq)]
enum Status {
    PreConfig,
    Config,
    PostConfig
}

#[derive(Props)]
pub struct DraggableProps<'a> {
    ptr_pos: &'a UseState<Option<(i32, i32)>>,
    z_index: &'a UseState<i32>,
    onpointerdown: EventHandler<'a, String>,
    data: &'a DragContent<'a>,
}

#[allow(non_snake_case)]
pub fn Draggable<'a>(cx: Scope<'a, DraggableProps<'a>>) -> Element {
    let active_record: &UseAtomRef<HashMap<String, bool>> = use_atom_ref(&cx, crate::drag_container::ACTIVE_RECORD);

    let pos_config_state = use_state(&cx, || Status::PreConfig);

    let position = use_state(&cx, || (0, 0));
    let start_pos = use_state(&cx, || Some((0, 0)));
    let local_z_index = use_state(&cx, || 199);

    let ptr_down_handler = move |event: UiEvent<PointerData>| {
        start_pos.set(Some((event.data.page_x - position.current().0, 
            event.data.page_y - position.current().1)));
        log::info!("ptr down on drag handle");
    };

    let active = active_record.read().get(&cx.props.data.title).map_or(false, |v| *v);

    let close_panel_action = move |title: String| {
        active_record.write().iter_mut().filter(|(k, _v)| **k == title)
        .next().map(|(_k, v)| *v = false);
    };

    if let Some(start_pos_data) = *start_pos.current() {
        match *cx.props.ptr_pos.current() {
            None => {
                start_pos.set(None);
            },
            Some(pos) => {
                position.set((
                    pos.0 - start_pos_data.0,
                    pos.1 - start_pos_data.1
                ));
            }
        };
    };


    let mut left = format!("{}px", position.current().0);
    let mut top = format!("{}px",position.current().1);
    let mut position_type = "absolute";
    let mut display = "";
    match *pos_config_state.current() {
        Status::PreConfig => {
            left = "".to_string();
            top = "".to_string();
            position_type = "relative";
            pos_config_state.set(Status::Config);
        },
        Status::Config => {
            let element_rect = get_element_by_id(&cx.props.data.title).get_bounding_client_rect();
            position.set((element_rect.x() as i32, element_rect.y() as i32));
            left = format!("{}px", position.current().0);
            top = format!("{}px",position.current().1);
            pos_config_state.set(Status::PostConfig);
        },
        Status::PostConfig => {
            display = match active {
                true => {
                    ""
                },
                false => {
                    "none"
                }
            }
        }
    };

    let draggable = rsx!{
        button {
            class: "draggable",
            style: "display: {display}; position: {position_type}; left: {left}; top: {top}; z-index: {local_z_index};",
            id: "{cx.props.data.title}",
            onpointerdown: move |_| {
                cx.props.z_index.set(**cx.props.z_index + 1);
                local_z_index.set(**cx.props.z_index);
                // /log::info!("pointer down for focus, index is {} local index is {}", **cx.props.z_index, local_z_index);
            },

            div {
                class: "drag-bar-wrapper",
                div {
                    class: "drag-bar",
                    onpointerdown: ptr_down_handler,
                    div {
                        class: "drag-bar-title",
                        div {
                            class: "arrows-box",
                            div {
                                class: "arrows",
                                div {
                                    class: "arrow arrow-left",
                                }
                                div {
                                    class: "arrow arrow-up",
                                }
                                div {
                                    class: "arrow arrow-right",
                                }
                                div {
                                    class: "arrow arrow-down",
                                }
                            }
                        }
                        span {
                            "{cx.props.data.title}"
                        }
                    }
                }
                cx.props.data.closeable.then(|| rsx!{

                    div {
                        class: "drag-bar-right",
                        div {
                            class: "drag-bar-button",
                            onpointerdown: move |evt| {
                                evt.cancel_bubble();
                                local_z_index.set(min_zero(**cx.props.z_index - 30));
                            },
                            "b"
                        }
                            div {
                                class: "drag-bar-button",
                                onpointerdown: move |_evt| close_panel_action(cx.props.data.title.clone()),
                                "x"
                            }
                        
                    }
                })
            }
            &cx.props.data.content
            
        }
    
    };

    cx.render(draggable)
}


fn get_element_by_id(id: &str) -> web_sys::Element {
    web_sys::window()
    .and_then(|win| win.document())
    .and_then(|doc| {
        doc.get_element_by_id(id)
    })
    .unwrap()
}

fn min_zero(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        0
    }
}


#[derive(Props)]
pub struct DragIconProps<'a> {
    onpointerdown: EventHandler<'a, UiEvent<PointerData>>,
    closeable: bool,
    close_setter: &'a UseState<bool>,
    title: &'a String,
}

#[allow(non_snake_case)]
pub fn DragIcon<'a>(cx: Scope<'a, DragIconProps<'a>>) -> Element {
    cx.render(rsx!{
        div {
            class: "drag-bar-wrapper",
            div {
                class: "drag-bar",
                onpointerdown: move |evt| cx.props.onpointerdown.call(evt),
                div {
                    class: "drag-bar-title",
                    span {
                        "{cx.props.title}"
                    }
                }
            }
            div {
                class: "drag-bar-right",
                div {
                    class: "drag-bar-button",
                    "b"
                }
                cx.props.closeable.then(|| rsx!{
                    div {
                        class: "drag-bar-button",
                        onpointerdown: move |_| cx.props.close_setter.set(false),
                        "x"
                    }
                })
            }
        }
        
    })
}
