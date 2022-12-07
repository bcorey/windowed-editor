use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::{ File, FileList, HtmlInputElement };

pub fn get_file(id: &str) -> Option<File> {
    log::info!("getting file");
    let input_el = match web_sys::window()
        .unwrap().document()
        .unwrap().get_element_by_id(id)
        {
            Some(element) => element,
            None => return None
        };
    
    let input_id = input_el.id().to_string();
    log::info!("{}", input_id);
    let input = input_el.dyn_into::<HtmlInputElement>()
    .map_err(|_| ())
    .expect("Should have found input field");

    if let Some(file_list) = input.files() {
        if let Some(file) = file_list.get(0) {
            return Some(file);
        };
    }
    log::info!("no file found");
    return None;
}