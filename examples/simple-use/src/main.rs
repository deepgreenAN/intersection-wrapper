use gloo_intersection::{IntersectionObserverHandler, IntersectionObserverOptions};
use gloo_utils::document;
use wasm_bindgen::JsCast;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let handler = IntersectionObserverHandler::new_with_options(
        move |entries, _| {
            entries.into_iter().for_each(|entry| {
                let id = entry.target().id();
                log::info!("intersecting with #{id}.");
            });
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
    // 状態として保持しないため
    std::mem::forget(handler);
}
