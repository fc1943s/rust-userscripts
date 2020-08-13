#![feature(option_result_contains)]

// #[macro_use]
// extern crate serde_derive;
//
use gloo::timers::callback::Interval;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{console, Document, HtmlElement, Window};

fn get_window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn get_document() -> Document {
    get_window()
        .document()
        .expect("should have a document on window")
}

fn collect_dom_token_list(dom_token_list: &web_sys::DomTokenList) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in 0..dom_token_list.length() {
        if let Some(item) = dom_token_list.item(i) {
            result.push(item);
        }
    }
    result
}

fn is_as_director_expanded() -> bool {
    let as_director_button = get_document()
        .query_selector(".artist-profile.page-artists a[data-wrapper-box=as-director]")
        .unwrap()
        .map(|element| element.dyn_into::<HtmlElement>().unwrap());

    if let Some(button) = as_director_button {
        let dom_token_list = button.class_list();
        let class_list = collect_dom_token_list(&dom_token_list);

        console::log_1(&format!("class_list: {:?}", class_list).into());

        if class_list.contains(&"see-more".into()) {
            button.click();
        }

        if class_list.contains(&"see-less".into()) {
            console::log_1(&format!("canceling: {:?}", true).into());
            return true;
        }
    }
    return false;
}

pub fn expand_as_director_timer() {
    let mut as_director_expanded = false;
    // let mut waiting_for_show_less = false;
    Interval::new(200, move || {
        if !as_director_expanded {
            as_director_expanded = is_as_director_expanded();
        }

        if as_director_expanded {
            console::log_1(&"parse_list".into());
            return;
        }

        // // let node_list = collect_node_list(&as_director_buttons);
        //
        // let as_director_button_classes: Option<Vec<String>> = as_director_button
        //     .map(|x| x.class_list())
        //     .map(|dom_token_list| collect_dom_token_list(&dom_token_list));
        //
        // // let as_director_buttons_classes: Vec<Vec<String>> = node_list
        // //     .into_iter()
        // //     .map(|node| node.dyn_into::<Element>().unwrap().class_list())
        // //     .map(|dom_token_list| collect_dom_token_list(&dom_token_list))
        // //     .collect();
        //
        // // console::log_1(&format!("as_director_buttons: {:?}", as_director_buttons).into());
        //
        // // console::log_1(&format!("x: {:?}", x).into());
        // console::log_1(&format!("names: {:?}", as_director_button_classes).into());

        // if as_director_button_classes
        //     .map(|classes| classes.contains(&"see-more".into()))
        //     .contains(&true)
        // {
        //     if let Some(element) = as_director_button {
        //         element.click(); //     }
        // }

        // if as_director_button_classes
        //     .map(|classes| classes.contains(&"see-less".into()))
        //     .contains(&true)
        // {
        //     // interval.cancel();
        // }
        // match (
        //     as_director_buttons_classes.contains("see-more"),
        //     as_director_buttons_classes.contains("see-less"),
        //     clicked,
        // ) {
        //     (true, false, false) => {
        //         console::log_1(&"---- first click!".into());
        //
        //         clicked = true;
        //         element.dyn_into::<HtmlElement>().unwrap().click();
        //     }
        //     (false, true, _) => {
        //         console::log_1(&"---- i should clean myself up here!".into());
        //     }
        //     _ => (),
        // }
    })
    .forget();
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = &window.document().expect("should have a document on window");

    // let body: HtmlElement = document.body().expect("document should have a body");

    expand_as_director_timer();

    // let stuff = vec![0, 0, 0, 0, 3];
    //
    // for num in stuff {
    //     let val = document.create_element("div")?;
    //     val.set_inner_html(&num.to_string());
    //     body.append_child(&val)?;
    // }

    console::log_1(&"---- TEST2!!!!!".into());

    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

// fn collect_node_list(node_list: &web_sys::NodeList) -> Vec<Box<web_sys::Node>> {
//     let mut result: Vec<Box<web_sys::Node>> = Vec::new();
//     for i in 0..node_list.length() {
//         if let Some(item) = node_list.item(i) {
//             result.push(Box::new(item));
//         }
//     }
//     result
// }
