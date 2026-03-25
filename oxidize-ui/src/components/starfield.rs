//! Parallax starfield background with twinkling stars
//! 
//! 3 star layers at different depths for parallax effect:
//! - Far layer: 0.1x pan speed (barely moves)
//! - Mid layer: 0.3x pan speed
//! - Near layer: 0.6x pan speed

use leptos::*;

/// Star positions for each layer (x%, y%, radius_px)
const FAR_STARS: &[(f64, f64, f64)] = &[
    (5.0, 10.0, 0.5), (12.0, 25.0, 0.5), (8.0, 40.0, 0.5), (15.0, 55.0, 0.5),
    (3.0, 70.0, 0.5), (18.0, 80.0, 0.5), (22.0, 8.0, 0.5), (28.0, 18.0, 0.5),
    (25.0, 35.0, 0.5), (30.0, 50.0, 0.5), (20.0, 65.0, 0.5), (32.0, 75.0, 0.5),
    (35.0, 5.0, 0.5), (40.0, 15.0, 0.5), (38.0, 30.0, 0.5), (42.0, 45.0, 0.5),
    (36.0, 60.0, 0.5), (45.0, 85.0, 0.5), (50.0, 12.0, 0.5), (55.0, 28.0, 0.5),
    (52.0, 42.0, 0.5), (58.0, 55.0, 0.5), (48.0, 70.0, 0.5), (60.0, 78.0, 0.5),
    (65.0, 8.0, 0.5), (70.0, 22.0, 0.5), (68.0, 38.0, 0.5), (72.0, 52.0, 0.5),
    (66.0, 68.0, 0.5), (75.0, 82.0, 0.5), (78.0, 15.0, 0.5), (82.0, 30.0, 0.5),
    (80.0, 48.0, 0.5), (85.0, 60.0, 0.5), (76.0, 75.0, 0.5), (88.0, 85.0, 0.5),
    (90.0, 5.0, 0.5), (95.0, 20.0, 0.5), (92.0, 35.0, 0.5), (97.0, 50.0, 0.5),
    (94.0, 65.0, 0.5), (98.0, 80.0, 0.5), (10.0, 90.0, 0.5), (25.0, 95.0, 0.5),
    (40.0, 92.0, 0.5), (55.0, 98.0, 0.5), (70.0, 93.0, 0.5), (85.0, 95.0, 0.5),
    (7.0, 17.0, 0.5), (17.0, 42.0, 0.5), (33.0, 3.0, 0.5), (43.0, 58.0, 0.5),
    (57.0, 63.0, 0.5), (73.0, 7.0, 0.5), (83.0, 73.0, 0.5), (93.0, 43.0, 0.5),
];

const MID_STARS: &[(f64, f64, f64)] = &[
    (12.0, 25.0, 1.0), (25.0, 35.0, 1.0), (40.0, 15.0, 1.0), (52.0, 42.0, 1.0),
    (68.0, 38.0, 1.0), (78.0, 15.0, 1.0), (80.0, 48.0, 1.0), (95.0, 20.0, 1.0),
    (10.0, 90.0, 1.0), (40.0, 92.0, 1.0), (60.0, 78.0, 1.0), (88.0, 85.0, 1.0),
    (18.0, 80.0, 1.0), (50.0, 12.0, 1.0), (72.0, 52.0, 1.0), (8.0, 40.0, 1.0),
];

const NEAR_STARS: &[(f64, f64, f64)] = &[
    (12.0, 25.0, 1.5), (25.0, 35.0, 1.5), (50.0, 12.0, 1.5), (68.0, 38.0, 1.5),
    (78.0, 15.0, 1.5), (80.0, 48.0, 1.5), (95.0, 20.0, 1.5), (40.0, 92.0, 1.5),
];

#[component]
pub fn Starfield(
    view_offset_x: ReadSignal<f64>,
    view_offset_y: ReadSignal<f64>,
) -> impl IntoView {
    // Far layer - parallax factor 0.1 (barely moves)
    let far_layer = {
        let ox = view_offset_x;
        let oy = view_offset_y;
        let stars: String = FAR_STARS.iter().enumerate().map(|(i, (bx, by, r))| {
            let delay = (i as f64 * 0.3) % 5.0;
            let twinkle_dur = 3.0 + (i as f64 * 0.7) % 3.0;
            format!(
                r#"<circle cx="{}%" cy="{}%" r="{}px" fill="white" opacity="0.4" style="animation: star-twinkle {:.1}s ease-in-out infinite {:.1}s;"/>"#,
                (*bx + ox.get() * 0.1).rem_euclid(100.0),
                (*by + oy.get() * 0.1).rem_euclid(100.0),
                r,
                twinkle_dur,
                delay
            )
        }).collect();
        stars
    };

    // Mid layer - parallax factor 0.3
    let mid_layer = {
        let stars: String = MID_STARS.iter().enumerate().map(|(i, (bx, by, r))| {
            let delay = (i as f64 * 0.4) % 4.0;
            let twinkle_dur = 3.5 + (i as f64 * 0.5) % 2.5;
            format!(
                r#"<circle cx="{}%" cy="{}%" r="{}px" fill="white" opacity="0.6" style="animation: star-twinkle {:.1}s ease-in-out infinite {:.1}s;"/>"#,
                (*bx + view_offset_x.get() * 0.3).rem_euclid(100.0),
                *by,
                r,
                twinkle_dur,
                delay
            )
        }).collect();
        stars
    };

    // Near layer - parallax factor 0.6 (moves noticeably)
    let near_layer = {
        let stars: String = NEAR_STARS.iter().enumerate().map(|(i, (bx, by, r))| {
            let delay = (i as f64 * 0.5) % 3.0;
            let twinkle_dur = 2.8 + (i as f64 * 0.3) % 2.0;
            format!(
                r#"<circle cx="{}%" cy="{}%" r="{}px" fill="white" opacity="0.9" style="animation: star-twinkle {:.1}s ease-in-out infinite {:.1}s;"/>"#,
                *bx,
                *by,
                r,
                twinkle_dur,
                delay
            )
        }).collect();
        stars
    };

    // Use move || in view to create reactive signals
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 0;" viewBox="0 0 100 100" preserveAspectRatio="none">
            {/* Far stars - faint, tiny, many */}
            <g style="animation: none;">
                {move || {
                    FAR_STARS.iter().enumerate().map(|(i, (bx, by, r))| {
                        let delay = (i as f64 * 0.3) % 5.0;
                        let twinkle_dur = 3.0 + (i as f64 * 0.7) % 3.0;
                        let cx = (*bx + view_offset_x.get() * 0.1).rem_euclid(100.0);
                        let cy = (*by + view_offset_y.get() * 0.1).rem_euclid(100.0);
                        format!(
                            r#"<circle cx="{}%" cy="{}%" r="{}px" fill="white" opacity="0.4" style="animation: star-twinkle {:.1}s ease-in-out infinite {:.1}s;"/>"#,
                            cx, cy, r, twinkle_dur, delay
                        )
                    }).collect::<String>()
                }}
            </g>
            {/* Mid stars - medium brightness */}
            <g style="animation: none;">
                {move || {
                    MID_STARS.iter().enumerate().map(|(i, (bx, by, r))| {
                        let delay = (i as f64 * 0.4) % 4.0;
                        let twinkle_dur = 3.5 + (i as f64 * 0.5) % 2.5;
                        let cx = (*bx + view_offset_x.get() * 0.3).rem_euclid(100.0);
                        format!(
                            r#"<circle cx="{}%" cy="{}%" r="{}px" fill="white" opacity="0.6" style="animation: star-twinkle {:.1}s ease-in-out infinite {:.1}s;"/>"#,
                            cx, *by, r, twinkle_dur, delay
                        )
                    }).collect::<String>()
                }}
            </g>
            {/* Near stars - bright, larger, fewer */}
            <g style="animation: none;">
                {move || {
                    NEAR_STARS.iter().enumerate().map(|(i, (bx, by, r))| {
                        let delay = (i as f64 * 0.5) % 3.0;
                        let twinkle_dur = 2.8 + (i as f64 * 0.3) % 2.0;
                        format!(
                            r#"<circle cx="{}%" cy="{}%" r="{}px" fill="white" opacity="0.9" style="animation: star-twinkle {:.1}s ease-in-out infinite {:.1}s;"/>"#,
                            *bx, *by, r, twinkle_dur, delay
                        )
                    }).collect::<String>()
                }}
            </g>
        </svg>
    }
}
