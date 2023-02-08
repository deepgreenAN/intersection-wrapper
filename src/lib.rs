mod ignore_first;
pub use ignore_first::IgnoreOnce;

use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// The wrapper of IntersectionObserverInit
#[derive(Clone)]
pub struct IntersectionObserverOptions {
    init: IntersectionObserverInit,
}

#[allow(clippy::new_without_default)]
impl IntersectionObserverOptions {
    /// Constructor of this options
    pub fn new() -> Self {
        Self {
            init: IntersectionObserverInit::new(),
        }
    }
    /// IntersectionObserverInit::root
    pub fn root(&mut self, val: Option<&Element>) {
        self.init.root(val);
    }
    /// IntersectionObserverInit::root_margin
    pub fn root_margin(&mut self, val: &str) {
        self.init.root_margin(val);
    }
    /// IntersectionObserverInit::threshold
    pub fn threshold(&mut self, val: &[f64]) {
        let array = val.iter().map(|x| JsValue::from_f64(*x)).collect::<Array>();
        self.init.threshold(array.as_ref());
    }
    /// Make builder.
    pub fn builder() -> IntersectionObserverOptionsBuilder {
        IntersectionObserverOptionsBuilder::new()
    }
}

pub struct IntersectionObserverOptionsBuilder {
    options: IntersectionObserverOptions,
}

#[allow(clippy::new_without_default)]
impl IntersectionObserverOptionsBuilder {
    pub fn new() -> Self {
        Self {
            options: IntersectionObserverOptions::new(),
        }
    }
    /// set root
    pub fn root(&mut self, val: Option<&Element>) -> &mut Self {
        self.options.root(val);
        self
    }
    /// set root_margin
    pub fn root_margin(&mut self, val: &str) -> &mut Self {
        self.options.root_margin(val);
        self
    }
    /// set threshold
    pub fn threshold(&mut self, val: &[f64]) -> &mut Self {
        self.options.threshold(val);
        self
    }
    /// Build options type.
    pub fn build(&mut self) -> IntersectionObserverOptions {
        self.options.clone()
    }
}

/// The wrapper of IntersectionObserver and the handler of js closure.
#[allow(clippy::type_complexity)]
#[derive(Debug)]
pub struct IntersectionObserverHandler {
    observer: IntersectionObserver,
    _callback: Option<Closure<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>>,
    // The flag whether intersection observer api has fired once or not.
}

impl IntersectionObserverHandler {
    /// Constructor of this handler.
    pub fn new<F: FnMut(Vec<&IntersectionObserverEntry>, &IntersectionObserver) + 'static>(
        mut callback: F,
    ) -> Result<IntersectionObserverHandler, JsValue> {
        // 引数を参照にするためのラッパー
        let callback_wrapper =
            move |entries: Vec<IntersectionObserverEntry>, observer: IntersectionObserver| {
                callback(entries.iter().collect(), &observer)
            };
        // Jsクロージャ．
        let closure = Closure::wrap(Box::new(callback_wrapper)
            as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);
        let observer = IntersectionObserver::new(closure.as_ref().unchecked_ref())?;
        Ok(Self {
            observer,
            _callback: Some(closure),
        })
    }
    /// Constructor with options.
    pub fn new_with_options<
        F: FnMut(Vec<&IntersectionObserverEntry>, &IntersectionObserver) + 'static,
    >(
        mut callback: F,
        options: &IntersectionObserverOptions,
    ) -> Result<IntersectionObserverHandler, JsValue> {
        let callback_wrapper =
            move |entries: Vec<IntersectionObserverEntry>, observer: IntersectionObserver| {
                callback(entries.iter().collect(), &observer)
            };
        let closure = Closure::wrap(Box::new(callback_wrapper)
            as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);
        let observer = IntersectionObserver::new_with_options(
            closure.as_ref().unchecked_ref(),
            &options.init,
        )?;
        Ok(Self {
            observer,
            _callback: Some(closure),
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
