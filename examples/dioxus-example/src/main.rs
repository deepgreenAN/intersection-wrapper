#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::{core::to_owned, prelude::*};
use gloo_console::log;
use gloo_intersection::{IntersectionObserverHandler, IntersectionObserverOptions};
use gloo_utils::document;
use wasm_bindgen::{JsCast, JsValue};

fn App(cx: Scope) -> Element {
    let _intersection_handler: &UseState<Option<IntersectionObserverHandler>> =
        use_state(&cx, || None);
    use_effect(&cx, (), {
        to_owned![_intersection_handler];
        |_| async move {
            let handler = IntersectionObserverHandler::new_with_options(
                {
                    move |entries, _| {
                        entries.into_iter().for_each(|entry| {
                            let id = entry.target().id();
                            log!(JsValue::from(&format!("intersecting with #{id}.")));
                        });
                    }
                },
                &IntersectionObserverOptions::builder()
                    .threshold(&[0.0])
                    .build(),
            )
            .unwrap();

            let target_node_list = document().query_selector_all(".intersection").unwrap();
            for i in 0..target_node_list.length() {
                let target_element = target_node_list.item(i).unwrap();
                handler.observe(&(target_element.unchecked_into()));
            }

            _intersection_handler.set(Some(handler));
        }
    });

    cx.render(rsx! {
        div { height: "900px", background_color:"grey"}
        div { class:"intersection", id: "target-1",height:"300px", "target 1"}
        div { height: "900px", background_color:"grey"}
        div { class:"intersection", id: "target-2", height:"300px", "target 2"}
        div { height: "900px", background_color:"grey"}
        div { class:"intersection", id: "target-3", height:"300px", "target 3"}
        div { height: "900px", background_color:"grey"}
    })
}

fn main() {
    dioxus::web::launch(App);
}
