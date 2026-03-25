//! Orbital path rings for the 8 planets
//! 
//! Self-contained component that renders 8 dashed circular orbits
//! representing the paths planets follow around the sun.

use leptos::*;

#[component]
pub fn OrbitalPaths() -> impl IntoView {
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 1;">
            // Mercury orbit (8%)
            <circle cx="50%%" cy="50%%" r="8%%" fill="none" stroke="#9ca3af" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.3"/>
            // Venus orbit (14%)
            <circle cx="50%%" cy="50%%" r="14%%" fill="none" stroke="#fbbf24" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            // Earth orbit (20%)
            <circle cx="50%%" cy="50%%" r="20%%" fill="none" stroke="#3b82f6" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            // Mars orbit (27%)
            <circle cx="50%%" cy="50%%" r="27%%" fill="none" stroke="#ef4444" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            // Jupiter orbit (38%)
            <circle cx="50%%" cy="50%%" r="38%%" fill="none" stroke="#f97316" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            // Saturn orbit (50%)
            <circle cx="50%%" cy="50%%" r="50%%" fill="none" stroke="#eab308" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            // Uranus orbit (62%)
            <circle cx="50%%" cy="50%%" r="62%%" fill="none" stroke="#06b6d4" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            // Neptune orbit (75%)
            <circle cx="50%%" cy="50%%" r="75%%" fill="none" stroke="#6366f1" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
        </svg>
    }
}
