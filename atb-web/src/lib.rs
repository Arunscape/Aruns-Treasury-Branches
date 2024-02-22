#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[cfg(feature = "ssr")]
pub mod apiroutes;
pub mod app;
pub mod components;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod pages;
pub mod serverfns;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
