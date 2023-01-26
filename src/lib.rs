use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// The wrapper of IntersectionObserver and the handler of js closure.
#[allow(clippy::type_complexity)]
#[derive(Debug)]
pub struct IntersectionObserverHandler {
    observer: IntersectionObserver,
    _callback: Option<Closure<dyn FnMut(&IntersectionObserverEntry)>>,
}

impl IntersectionObserverHandler {
    /// Constructor of this handler.
    pub fn new<F: FnMut(&IntersectionObserverEntry) + 'static>(
        callback: F,
    ) -> Result<IntersectionObserverHandler, JsValue> {
        let callback =
            Closure::wrap(Box::new(callback) as Box<dyn FnMut(&IntersectionObserverEntry)>);
        let observer = IntersectionObserver::new(callback.as_ref().unchecked_ref())?;
        Ok(Self {
            observer,
            _callback: Some(callback),
        })
    }
    /// Constructor with options.
    pub fn new_with_options<F: FnMut(&IntersectionObserverEntry) + 'static>(
        callback: F,
        options: &IntersectionObserverInit,
    ) -> Result<IntersectionObserverHandler, JsValue> {
        let callback =
            Closure::wrap(Box::new(callback) as Box<dyn FnMut(&IntersectionObserverEntry)>);
        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), options)?;
        Ok(Self {
            observer,
            _callback: Some(callback),
        })
    }
    /// IntersectionObserver::root
    pub fn root(&self) -> Option<Element> {
        self.observer.root()
    }
    /// IntersectionObserver::root_margin
    pub fn root_margin(&self) -> String {
        self.observer.root_margin()
    }
    /// IntersectionObserver::thresholds as Vec<f64>
    pub fn thresholds(&self) -> Vec<f64> {
        self.observer
            .thresholds()
            .to_vec()
            .into_iter()
            .map(|js_value| js_value.as_f64().unwrap_throw())
            .collect()
    }
    /// IntersectionObserver::disconnect
    pub fn disconnect(&self) {
        self.observer.disconnect();
    }
    /// IntersectionsObserver::observe
    pub fn observe(&self, target: &Element) {
        self.observer.observe(target);
    }
    /// IntersectionsObserver::take_records as Vec<IntersectionObserverEntry>
    pub fn take_records(&self) -> Vec<IntersectionObserverEntry> {
        self.observer
            .take_records()
            .to_vec()
            .into_iter()
            .map(|js_value| js_value.into())
            .collect::<Vec<IntersectionObserverEntry>>()
    }
    /// IntersectionObserver::unobserve
    pub fn unobserve(&self, target: &Element) {
        self.observer.unobserve(target);
    }
    /// inner observer
    pub fn observer(&self) -> &IntersectionObserver {
        &self.observer
    }
}

impl Drop for IntersectionObserverHandler {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
