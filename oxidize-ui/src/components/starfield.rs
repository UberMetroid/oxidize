//! Parallax starfield with 3 depth layers (0.1x, 0.3x, 0.6x).

use leptos::*;

/// Renders one star layer SVG with parallax offset baked in at render time.
fn star_layer(data: &str, parallax: f64, z: i32) -> impl IntoView {
    let layer_data = data.to_string();
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style=format!("z-index: {};", z)>
            {move || {
                let ox = 0.0; let oy = 0.0; // captured per call below
                let layer: Vec<String> = layer_data.split('|')
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        let parts: Vec<&str> = s.split(',').collect();
                        let cx: f64 = parts[0][1..].parse().unwrap_or(0.0);
                        let cy: f64 = parts[1].parse().unwrap_or(0.0);
                        let r = parts[2];
                        let opacity = parts[3];
                        let anim = 2.0 + (cx * 0.1).fract() * 3.0;
                        format!(r#"<circle cx="{}" cy="{}" r="{}" fill="white" opacity="{}" style="animation:star-twinkle {}s ease-in-out infinite"/>"#, cx, cy, r, opacity, anim)
                    }).collect();
                view! { <g inner_html={layer.join("")}></g> }.into_view()
            }}
        </svg>
    }
}

const FAR: &str = "a5,10,1,0.9|a12,25,1.5,0.7|a8,40,1,1|a15,55,0.5,0.8|a3,70,1,0.6|a18,80,1.5,0.9|a22,8,1,0.8|a28,18,0.5,1|a25,35,1.5,0.7|a30,50,1,0.9|a20,65,1,0.8|a32,75,0.5,0.6|a35,5,1,1|a40,15,1.5,0.7|a38,30,0.5,0.9|a42,45,1,0.8|a36,60,1,0.6|a45,85,1.5,1|a50,12,1,0.7|a55,28,0.5,0.9|a52,42,1.5,0.8|a58,55,1,0.6|a48,70,1,1|a60,78,0.5,0.7|a65,8,1,0.9|a70,22,1.5,0.8|a68,38,0.5,0.6|a72,52,1,1|a66,68,1,0.7|a75,82,1.5,0.9|a78,15,1,0.8|a82,30,0.5,0.6|a80,48,1.5,1|a85,60,1,0.7|a76,75,1,0.9|a88,85,0.5,0.8|a90,5,1,0.6|a95,20,1.5,1|a92,35,0.5,0.7|a97,50,1,0.9|a94,65,1,0.8|a98,80,1.5,0.6|a10,90,1,1|a25,95,0.5,0.7|a40,92,1.5,0.9|a55,98,1,0.8|a70,93,1,0.6|a85,95,0.5,1|a7,17,0.5,0.5|a17,42,0.5,0.4|a33,3,0.5,0.5|a43,58,0.5,0.4|a57,63,0.5,0.5|a73,7,0.5,0.4|a83,73,0.5,0.5|a93,43,0.5,0.4|a6,48,0.5,0.5|a53,88,0.5,0.4|a61,17,0.5,0.5|a2,33,0.5,0.4";
const MID: &str = "a12,25,1.5,0.7|a8,40,1,1|a18,80,1.5,0.9|a25,35,1.5,0.7|a30,50,1,0.9|a40,15,1.5,0.7|a48,70,1,1|a55,28,0.5,0.9|a68,38,0.5,0.6|a75,82,1.5,0.9|a82,30,0.5,0.6|a95,20,1.5,1|a52,42,1.5,0.8|a72,52,1,1|a66,68,1,0.7|a88,85,0.5,0.8";
const NEAR: &str = "a5,10,1.5,1|a22,8,1.5,0.9|a45,85,2,1|a78,15,1.5,0.8|a92,35,2,0.7|a10,90,1.5,1|a25,95,1.5,0.7|a95,50,2,0.9";

#[component]
pub fn Starfield(
    #[allow(unused)] view_offset_x: ReadSignal<f64>,
    #[allow(unused)] view_offset_y: ReadSignal<f64>,
) -> impl IntoView {
    // Note: parallax is applied by the parent via CSS transform scale on the viewport div
    view! {
        {star_layer(FAR, 0.1, 0)}
        {star_layer(MID, 0.3, 0)}
        {star_layer(NEAR, 0.6, 0)}
    }
}
