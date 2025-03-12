#![recursion_limit = "256"]

use cfg_if::cfg_if;

pub mod app;
pub mod report;
pub mod home;
mod components;


#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
     _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(crate::app::App);
}
