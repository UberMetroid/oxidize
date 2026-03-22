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
            <div class="flex flex-col h-full items-center justify-center relative z-10 pointer-events-none">
                <h1 class="text-6xl font-black tracking-widest text-theme-primary">
                    "OXIDIZE"
                </h1>
                <p class="text-xl mt-4 opacity-70 tracking-widest">
                    "INITIALIZING THE DYSON PROTOCOL..."
                </p>
                <div class="flex gap-2 mt-8 pointer-events-auto">
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
                                    class=format!("w-8 h-8 rounded-full border-2 border-white border-opacity-20 hover:scale-110 transition-all {}", bg)
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
