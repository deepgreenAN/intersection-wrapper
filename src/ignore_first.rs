use crate::{IntersectionObserver, IntersectionObserverHandler, IntersectionObserverOptions};

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::IntersectionObserverEntry;

/// The extension to ignore the first fire of intersection(the firing is probably when it is loaded).
pub trait IgnoreOnce {
    /// Constructor of Handler that ignore once.
    fn new_ignore_once<F: FnMut(Vec<&IntersectionObserverEntry>, &IntersectionObserver) + 'static>(
        callback: F,
    ) -> Result<IntersectionObserverHandler, JsValue>;

    /// Constructor of Handler with options, which closure ignore once.
    fn new_ignore_once_with_options<
        F: FnMut(Vec<&IntersectionObserverEntry>, &IntersectionObserver) + 'static,
    >(
        callback: F,
        options: &IntersectionObserverOptions,
    ) -> Result<IntersectionObserverHandler, JsValue>;
}

impl IgnoreOnce for IntersectionObserverHandler {
    fn new_ignore_once<
        F: FnMut(Vec<&IntersectionObserverEntry>, &IntersectionObserver) + 'static,
    >(
        mut callback: F,
    ) -> Result<IntersectionObserverHandler, JsValue> {
        let is_once_fired = Rc::new(Cell::new(false));
        let callback_wrapper =
            move |entries: Vec<IntersectionObserverEntry>, observer: IntersectionObserver| {
                if is_once_fired.get() {
                    callback(entries.iter().collect(), &observer);
                }
                is_once_fired.set(true);
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
    fn new_ignore_once_with_options<
        F: FnMut(Vec<&IntersectionObserverEntry>, &IntersectionObserver) + 'static,
    >(
        mut callback: F,
        options: &IntersectionObserverOptions,
    ) -> Result<IntersectionObserverHandler, JsValue> {
        let is_once_fired = Rc::new(Cell::new(false));
        let callback_wrapper =
            move |entries: Vec<IntersectionObserverEntry>, observer: IntersectionObserver| {
                if is_once_fired.get() {
                    callback(entries.iter().collect(), &observer);
                }
                is_once_fired.set(true);
            };
        // Jsクロージャ．
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
}
