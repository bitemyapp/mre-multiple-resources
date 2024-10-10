pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    mount_to_body(App);
}
