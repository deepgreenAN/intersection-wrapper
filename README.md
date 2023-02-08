# intersection wrapper
The wrapper of web-sys IntersectionObserver

## usage
```
gloo-intersection = {git = "https://github.com/deepgreenAN/intersection-wrapper"}
```

```rust
use gloo_intersection::{IntersectionObserverHandler, IntersectionObserverOptions};
use wasm_bindgen::JsCast;

let handler = IntersectionObserverHandler::new_with_options(
    move |entries, _observer| {
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

let target_node_list = gloo_utils::document().query_selector_all(".intersection").unwrap();
for i in 0..target_node_list.length() {
    let target_element = target_node_list.item(i).unwrap();
    handler.observe(&(target_element.unchecked_into()));
}

// RAII
std::mem::forget(handler);
```