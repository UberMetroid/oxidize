use leptos::*;
use wasm_bindgen::JsCast;

#[component]
fn App() -> impl IntoView {
    let (theme, set_theme) = create_signal("orange".to_string());

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(t)) = storage.get_item("color-theme") {
                    set_theme.set(t.clone());
                } else {
                    let _ = storage.set_item("color-theme", &theme.get());
                }
            }
        }
    });

    create_effect(move |_| {
        let t = theme.get();
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("class", &format!("theme-{}", t));
                }
            }
        }
    });

    view! {
        <div class="flex flex-col h-full bg-app-bg text-app-text overflow-hidden transition-all duration-500 relative font-mono">
            {/* TOP HEADER */}
            <div class="flex flex-col items-center pt-8 relative z-10 pointer-events-none shrink-0">
                <h1 class="text-5xl sm:text-6xl font-black tracking-widest text-theme-primary">
                    "OXIDIZE"
                </h1>
                <p class="text-sm sm:text-base mt-2 opacity-70 tracking-widest text-center px-4">
                    "INITIALIZING THE DYSON PROTOCOL... AWAITING CONSTRUCT."
                </p>
            </div>

            {/* MIDDLE (3D SPACE) */}
            <div class="flex-1 pointer-events-none"></div>

            {/* BOTTOM FOOTER */}
            <div class="w-full flex justify-center pb-8 shrink-0 relative z-10 pointer-events-auto">
                <div class="flex gap-4">
                    {
                        let themes = ["red", "orange", "yellow", "green", "blue", "purple"];
                        themes.into_iter().map(|t| {
                            let bg = match t {
                                "red" => "bg-red-500",
                                "orange" => "bg-orange-500",
                                "yellow" => "bg-yellow-400",
                                "green" => "bg-green-500",
                                "blue" => "bg-blue-500",
                                "purple" => "bg-purple-500",
                                _ => "bg-gray-500"
                            };
                            let t_str = t.to_string();
                            view! {
                                <button 
                                    on:click=move |_| {
                                        set_theme.set(t_str.clone());
                                        if let Some(window) = web_sys::window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let _ = storage.set_item("color-theme", &t_str);
                                            }
                                        }
                                    }
                                    class=format!("w-10 h-10 clip-hexagon hover:scale-110 hover:brightness-125 transition-all cursor-pointer shadow-[0_0_15px_rgba(255,255,255,0.1)] hover:shadow-[0_0_20px_currentColor] {}", bg)
                                />
                            }
                        }).collect_view()
                    }
                </div>
            </div>
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
