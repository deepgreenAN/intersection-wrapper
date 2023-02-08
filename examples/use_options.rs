use gloo_intersection::{IntersectionObserverHandler, IntersectionObserverOptions};

fn main() {
    let _handler = IntersectionObserverHandler::new_with_options(
        move |_, _| {},
        &IntersectionObserverOptions::builder()
            .threshold(&[0.0])
            .build(),
    )
    .unwrap();
}
