//! Asteroid belt rendering
//! 
//! Self-contained component that renders 5 concentric rings
//! and ~30 scattered asteroid dots between Mars and Jupiter.

use leptos::*;

#[component]
pub fn AsteroidBelt() -> impl IntoView {
    let asteroid_belt = "\
<circle cx=\"50%\" cy=\"50%\" r=\"30%\" fill=\"none\" stroke=\"#555\" stroke-width=\"1%\" opacity=\"0.05\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"31%\" fill=\"none\" stroke=\"#666\" stroke-width=\"1.5%\" opacity=\"0.08\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"32%\" fill=\"none\" stroke=\"#777\" stroke-width=\"2%\" opacity=\"0.1\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"33%\" fill=\"none\" stroke=\"#666\" stroke-width=\"2.5%\" opacity=\"0.12\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"34%\" fill=\"none\" stroke=\"#555\" stroke-width=\"3%\" opacity=\"0.08\"/>\
<circle cx=\"52%\" cy=\"46%\" r=\"1px\" fill=\"#888\" opacity=\"0.6\"/>\
<circle cx=\"48%\" cy=\"53%\" r=\"1.5px\" fill=\"#777\" opacity=\"0.5\"/>\
<circle cx=\"55%\" cy=\"48%\" r=\"0.5px\" fill=\"#999\" opacity=\"0.7\"/>\
<circle cx=\"51%\" cy=\"55%\" r=\"2px\" fill=\"#666\" opacity=\"0.4\"/>\
<circle cx=\"46%\" cy=\"47%\" r=\"1px\" fill=\"#888\" opacity=\"0.55\"/>\
<circle cx=\"54%\" cy=\"43%\" r=\"1.5px\" fill=\"#777\" opacity=\"0.6\"/>\
<circle cx=\"49%\" cy=\"57%\" r=\"0.5px\" fill=\"#999\" opacity=\"0.5\"/>\
<circle cx=\"56%\" cy=\"52%\" r=\"1px\" fill=\"#666\" opacity=\"0.65\"/>\
<circle cx=\"47%\" cy=\"51%\" r=\"2px\" fill=\"#888\" opacity=\"0.45\"/>\
<circle cx=\"53%\" cy=\"45%\" r=\"0.5px\" fill=\"#777\" opacity=\"0.55\"/>\
<circle cx=\"50%\" cy=\"49%\" r=\"1.5px\" fill=\"#999\" opacity=\"0.5\"/>\
<circle cx=\"44%\" cy=\"50%\" r=\"1px\" fill=\"#666\" opacity=\"0.4\"/>\
<circle cx=\"57%\" cy=\"49%\" r=\"0.5px\" fill=\"#888\" opacity=\"0.6\"/>\
<circle cx=\"48%\" cy=\"44%\" r=\"2px\" fill=\"#777\" opacity=\"0.5\"/>\
<circle cx=\"52%\" cy=\"56%\" r=\"1px\" fill=\"#999\" opacity=\"0.45\"/>\
<circle cx=\"45%\" cy=\"54%\" r=\"1.5px\" fill=\"#666\" opacity=\"0.55\"/>\
<circle cx=\"55%\" cy=\"54%\" r=\"0.5px\" fill=\"#888\" opacity=\"0.4\"/>\
<circle cx=\"51%\" cy=\"43%\" r=\"1px\" fill=\"#777\" opacity=\"0.6\"/>\
<circle cx=\"49%\" cy=\"58%\" r=\"1.5px\" fill=\"#999\" opacity=\"0.35\"/>\
<circle cx=\"53%\" cy=\"57%\" r=\"0.5px\" fill=\"#666\" opacity=\"0.5\"/>\
<circle cx=\"47%\" cy=\"44%\" r=\"1px\" fill=\"#888\" opacity=\"0.45\"/>\
<circle cx=\"54%\" cy=\"47%\" r=\"2px\" fill=\"#777\" opacity=\"0.4\"/>\
<circle cx=\"48%\" cy=\"57%\" r=\"0.5px\" fill=\"#999\" opacity=\"0.55\"/>\
<circle cx=\"52%\" cy=\"52%\" r=\"1px\" fill=\"#666\" opacity=\"0.5\"/>\
<circle cx=\"46%\" cy=\"52%\" r=\"1.5px\" fill=\"#888\" opacity=\"0.35\"/>\
<circle cx=\"55%\" cy=\"51%\" r=\"0.5px\" fill=\"#777\" opacity=\"0.45\"/>\
<circle cx=\"50%\" cy=\"46%\" r=\"1px\" fill=\"#999\" opacity=\"0.5\"/>\
<circle cx=\"50%\" cy=\"55%\" r=\"0.5px\" fill=\"#666\" opacity=\"0.4\"/>";

    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 5;" inner_html={asteroid_belt}></svg>
    }
}
