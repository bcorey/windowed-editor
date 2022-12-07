use dioxus::prelude::*;

#[derive(Debug)]
pub struct DragContent<'a> {
    pub content: Element<'a>,
    pub title: String,
    pub closeable: bool,
    pub active: bool,
}

impl <'a> DragContent<'a> {
    pub fn new(content: Element<'a>, title: String, closeable: bool) -> Self {

        DragContent { 
            content: content, 
            title: title, 
            closeable: closeable,
            active: true,
        }
    }

    pub fn new_extra(content: Element<'a>, title: String) -> Self {

        DragContent { 
            content: content, 
            title: title, 
            closeable: true,
            active: true,
        }
    }

    pub fn new_hidden(content: Element<'a>, title: String) -> Self {

        DragContent { 
            content: content, 
            title: title, 
            closeable: true,
            active: false,
        }
    }

    pub fn close(&mut self) {
        self.active = false;
    }

}

/*
#[derive(Props)]
struct DragContentProps<'a> {
    ptr_pos: &'a UseState<Option<(i32, i32)>>,
    z_index: &'a UseState<i32>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
fn DragContent<'a>(cx: Scope<'a, DragContentProps<'a>>) -> Element {

    let active = use_state(&cx, || true);

    cx.render(rsx!{
        /*Draggable {
            key: "hello",
            ptr_pos: cx.props.ptr_pos,
            z_index: cx.props.z_index,
            data: item
        }*/
        div {

        }
    })
}*/