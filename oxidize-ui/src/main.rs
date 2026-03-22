use leptos::*;
use wasm_bindgen::JsCast;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col h-full items-center justify-center relative z-10 text-white font-mono pointer-events-none">
            <h1 class="text-6xl font-black tracking-widest text-orange-500 drop-shadow-[0_0_15px_rgba(249,115,22,0.8)]">
                "OXIDIZE"
            </h1>
            <p class="text-xl mt-4 opacity-70 tracking-widest">
                "INITIALIZING THE DYSON PROTOCOL..."
            </p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("root")
        .expect("could not find #root element");
    mount_to(root.unchecked_into(), || view! { <App/> })
}
