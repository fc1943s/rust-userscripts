// #[macro_use]
// extern crate serde_derive;
//
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let stuff = vec![0, 0, 0, 0, 3];

    for num in stuff {
        let val = document.create_element("div")?;
        val.set_inner_html(&num.to_string());
        body.append_child(&val)?;
    }

    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
