use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use dioxus::prelude::*;

const MARK_PATH: &str = "M231.5 0C359.354 0 463 103.646 463 231.5C463 359.354 359.354 463 231.5 463C103.646 463 0 359.354 0 231.5C0 103.646 103.646 0 231.5 0ZM241.736 96.7617C222.831 96.7617 206.257 99.9521 192.017 106.333C177.776 112.714 166.604 121.795 158.501 133.575C150.644 145.356 146.716 159.59 146.716 176.279C146.716 189.287 148.925 200.209 153.345 209.044C157.764 217.879 163.535 225.242 170.655 231.132C178.021 237.022 186.001 242.053 194.595 246.226C203.434 250.152 212.15 253.588 220.743 256.533C229.582 259.233 237.562 262.178 244.683 265.368C252.049 268.313 257.942 271.873 262.361 276.045C266.781 279.972 268.99 285.003 268.99 291.139C268.99 297.765 265.921 303.041 259.783 306.968C253.89 310.649 245.419 312.49 234.37 312.49C221.357 312.49 209.694 310.035 199.382 305.127C189.07 299.973 179.371 292.61 170.286 283.039L128.669 325.007C141.928 339.241 156.537 349.672 172.496 356.299C188.701 362.925 207.731 366.238 229.583 366.238C262.484 366.238 288.019 359.121 306.188 344.887C324.603 330.652 333.811 310.527 333.811 284.512C333.811 271.259 331.601 259.97 327.182 250.644C322.762 241.317 316.869 233.586 309.503 227.45C302.383 221.315 294.526 216.283 285.933 212.356C277.339 208.43 268.622 204.994 259.783 202.049C251.19 199.104 243.21 196.282 235.844 193.582C228.723 190.637 222.953 187.324 218.533 183.643C214.114 179.961 211.904 175.175 211.904 169.285C211.904 163.395 214.483 158.855 219.639 155.664C224.795 152.474 231.915 150.878 241 150.878C250.576 150.878 259.415 152.842 267.518 156.769C275.62 160.695 283.6 166.585 291.457 174.438L333.442 132.839C323.13 121.55 309.994 112.714 294.035 106.333C278.076 99.952 260.642 96.7617 241.736 96.7617Z";

#[component]
pub fn BrandMark(#[props(default = "")] class: &'static str) -> Element {
    let class = match class {
        "" => "shrink-0 text-accent",
        "h-9 w-9" => "shrink-0 text-accent h-9 w-9",
        "h-7 w-7" => "shrink-0 text-accent h-7 w-7",
        "h-3.5 w-3.5 opacity-50" => "shrink-0 text-accent h-3.5 w-3.5 opacity-50",
        "splash-logo h-16 w-16 sm:h-20 sm:w-20" => {
            "shrink-0 text-accent splash-logo h-16 w-16 sm:h-20 sm:w-20"
        }
        _ => "shrink-0 text-accent",
    };

    rsx! {
        svg {
            class,
            view_box: "0 0 463 463",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            path {
                d: MARK_PATH,
                fill: "currentColor",
                fill_rule: "evenodd",
                clip_rule: "evenodd",
            }
        }
    }
}

pub fn favicon_svg(accent: &'static str) -> &'static str {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, &'static str>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::with_capacity(12)));
    let mut map = cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(svg) = map.get(accent).copied() {
        return svg;
    }
    let owned = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 463 463' fill='none'><path fill-rule='evenodd' clip-rule='evenodd' d='{MARK_PATH}' fill='{accent}'/></svg>"
    );
    let leaked: &'static str = Box::leak(owned.into_boxed_str());
    map.insert(accent, leaked);
    leaked
}
