use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    leptos::mount_to_body(move |cx| {
        leptos::view! { cx, <App/> }
    });
}
