use dioxus::prelude::*;
use dioxus::events::PointerData;
use dioxus::core::{UiEvent};
use std::collections::HashMap;
use dioxus::fermi::*;
use crate::draggable::*;
use crate::drag_content::*;


pub static ACTIVE_RECORD: AtomRef<HashMap<String, bool>> = |_| HashMap::new();


#[derive(PartialEq)]
enum Status {
    PreConfig,
    Config,
    PostConfig
}

#[derive(Props)]
pub struct DragAreaProps<'a> {
    content: Vec<DragContent<'a>>,
}

#[allow(non_snake_case)]
pub fn DragArea<'a>(cx: Scope<'a, DragAreaProps<'a>>) -> Element {

    let ptr_pos: &UseState<Option<(i32, i32)>> = use_state(&cx, || None);

    //log::info!("{:?}", **ptr_pos);
    let z_index = use_state(&cx, || 200);

    let pointer_move_handler = move |event: UiEvent<PointerData>| {
        if let Some(_) = *ptr_pos.current() {
            ptr_pos.set(Some((event.data.page_x, event.data.page_y)));
        };      
        log::info!("ptr move on container");
    };

    let pointer_up_handler = move |_| {
        ptr_pos.set(None);
    };

    let pointer_down_handler = move |event: UiEvent<PointerData> | {
        ptr_pos.set(Some((event.data.page_x, event.data.page_y)));
    };

    //let active_record: &UseRef<HashMap<String, bool>> = use_ref(&cx, || HashMap::new());
    let active_record: &UseAtomRef<HashMap<String, bool>> = use_atom_ref(&cx, ACTIVE_RECORD);

    //active_record.read().values().then(|v| log::info!("{}", v));
    //active_record.read().values().try_for_each(|v| log::info!("{}", v));

    //log::info!("record status: {:?}", active_record.read().iter().filter(|(k, _v)| **k == "image 0".to_string()).next());
    //log::info!("actual status: {:?}", cx.props.content.iter().filter(|item| item.title == "image 0".to_string()).next().map(|item| item.active));

    let close_panel_action = move |title: String| {
        active_record.write().iter_mut().filter(|(k, _v)| **k == title)
        .next().map(|(_k, v)| *v = false);
    };

    let first_load = use_state(&cx, || Status::PreConfig);
    match **first_load {
        Status::PreConfig => first_load.set(Status::Config),
        Status::Config => {
            for item in &cx.props.content {
                active_record.write().insert(item.title.clone(), item.active);
            }
            log::info!("first load");
            first_load.set(Status::PostConfig);
        },
        Status::PostConfig => ()
    }
    
    cx.render(rsx!{
        div {
            class: "drag-area",
            prevent_default: "onpointermove",
            onpointerdown: pointer_down_handler,
            onpointermove: move |evt| {
                evt.cancel_bubble();
                pointer_move_handler(evt);
            },
            onpointerup: pointer_up_handler,

            cx.props.content.iter().map(|item| rsx!{
                Draggable {
                    key: "{item.title}",
                    ptr_pos: ptr_pos,
                    z_index: z_index,
                    onpointerdown: close_panel_action,
                    data: item,
                }
            }),
        }
    })
}