pub mod app;
pub mod components;
pub mod pages;
pub mod sections;
pub mod utils;

pub mod types {
    pub mod project;
}

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

