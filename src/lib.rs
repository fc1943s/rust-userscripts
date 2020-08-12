// #[macro_use]
// extern crate serde_derive;
//
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window: web_sys::Window = web_sys::window().expect("no global `window` exists");
    let document: web_sys::Document = window.document().expect("should have a document on window");
    let body: web_sys::HtmlElement = document.body().expect("document should have a body");

    let stuff = vec![0, 0, 0, 0, 3];

    for num in stuff {
        let val = document.create_element("div")?;
        val.set_inner_html(&num.to_string());
        body.append_child(&val)?;
    }

    println!("---- TEST1!!!!!");
    // unsafe.
    log("---- TEST2!!!!!");

    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
