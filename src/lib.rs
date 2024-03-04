pub mod app;
pub mod components;
pub mod pages;
pub mod sections;
pub mod utils;

pub mod types {
    pub mod project;
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}

