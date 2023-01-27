#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::{core::to_owned, prelude::*};
use gloo_console::log;
use gloo_intersection::{IntersectionObserverHandler, IntersectionObserverOptions};
use gloo_utils::document;
use wasm_bindgen::JsValue;

fn App(cx: Scope) -> Element {
    let _intersection_handler: &UseState<Option<IntersectionObserverHandler>> =
        use_state(&cx, || None);
    use_effect(&cx, (), {
        to_owned![_intersection_handler];
        |_| async move {
            let handler = IntersectionObserverHandler::new_with_options(
                {
                    move |_| {
                        log!(JsValue::from("intersection observer fired."));
                    }
                },
                &IntersectionObserverOptions::builder()
                    .threshold(&[0.0, 0.5, 1.0])
                    .build(),
            )
            .unwrap();
            handler.observe(
                document()
                    .query_selector("#intersection")
                    .unwrap()
                    .unwrap()
                    .as_ref(),
            );
            _intersection_handler.set(Some(handler));
        }
    });

    cx.render(rsx! {
        div { height: "700px", background_color:"grey"}
        div { id:"intersection", height:"300px", "target"}
        div { height: "700px", background_color:"grey"}
    })
}

fn main() {
    dioxus::web::launch(App);
}
