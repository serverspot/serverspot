import fs from "fs";
import * as icons from "@hugeicons/core-free-icons";

const selected = {
  DashboardSquare01: "DashboardSquare01Icon",
  Layers01: "Layers01Icon",
  UserGroup: "UserGroupIcon",
  ChartLineData01: "ChartLineData01Icon",
  Notification03: "Notification03Icon",
  Settings01: "Settings01Icon",
  Add01: "Add01Icon",
  Wallet01: "Wallet01Icon",
  Search01: "Search01Icon",
  ArrowDown01: "ArrowDown01Icon",
  ShoppingCart01: "ShoppingCart01Icon",
  Store01: "Store01Icon",
  Message01: "Message01Icon",
  Ticket01: "Ticket01Icon",
  News: "NewsIcon",
  BookOpen01: "BookOpen01Icon",
  Discord: "DiscordIcon",
  Globe02: "Globe02Icon",
  CustomerService01: "CustomerService01Icon",
  AnalyticsUp: "AnalyticsUpIcon",
  Package01: "Package01Icon",
  HelpCircle: "HelpCircleIcon",
  AiChat01: "AiChat01Icon",
};

function esc(s) {
  return String(s).replace(/\\/g, "\\\\").replace(/"/g, '\\"');
}

function num(v) {
  if (v == null) return null;
  return String(v);
}

function nodeRust(el) {
  const [tag, attrs] = el;
  const opt = (v) => (v != null ? `Some("${esc(v)}")` : "None");
  return `    HugeIconNode {
        tag: "${tag}",
        d: ${opt(attrs.d)},
        cx: ${opt(num(attrs.cx))},
        cy: ${opt(num(attrs.cy))},
        r: ${opt(num(attrs.r ?? attrs.rx))},
        x: ${opt(num(attrs.x))},
        y: ${opt(num(attrs.y))},
        width: ${opt(num(attrs.width))},
        height: ${opt(num(attrs.height))},
        x1: ${opt(num(attrs.x1))},
        y1: ${opt(num(attrs.y1))},
        x2: ${opt(num(attrs.x2))},
        y2: ${opt(num(attrs.y2))},
        points: ${opt(attrs.points)},
        fill: ${attrs.fill && attrs.fill !== "none" ? opt(attrs.fill) : "None"},
        stroke_width: ${opt(num(attrs.strokeWidth))},
    }`;
}

let out = `use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct HugeIconNode {
    pub tag: &'static str,
    pub d: Option<&'static str>,
    pub cx: Option<&'static str>,
    pub cy: Option<&'static str>,
    pub r: Option<&'static str>,
    pub x: Option<&'static str>,
    pub y: Option<&'static str>,
    pub width: Option<&'static str>,
    pub height: Option<&'static str>,
    pub x1: Option<&'static str>,
    pub y1: Option<&'static str>,
    pub x2: Option<&'static str>,
    pub y2: Option<&'static str>,
    pub points: Option<&'static str>,
    pub fill: Option<&'static str>,
    pub stroke_width: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct HugeIconData {
    pub name: &'static str,
    pub nodes: &'static [HugeIconNode],
}

`;

for (const [rustName, exportName] of Object.entries(selected)) {
  const data = icons[exportName];
  if (!data) throw new Error(`missing ${exportName}`);
  const snake = rustName
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .replace(/([A-Za-z])([0-9])/g, "$1_$2")
    .toUpperCase();
  out += `pub const ${snake}: HugeIconData = HugeIconData {\n`;
  out += `    name: "${exportName}",\n`;
  out += `    nodes: &[\n`;
  for (const el of data) {
    out += `${nodeRust(el)},\n`;
  }
  out += `    ],\n};\n\n`;
}

out += `#[component]
pub fn HugeIcon(
    icon: HugeIconData,
    #[props(default = 16)]
    size: u32,
    #[props(default, into)]
    class: String,
    #[props(default = 1.5)]
    stroke_width: f32,
) -> Element {
    let px = size.to_string();
    rsx! {
        svg {
            class: "shrink-0 {class}",
            width: "{px}",
            height: "{px}",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            for node in icon.nodes {
                {
                    let sw = node.stroke_width.unwrap_or("1.5");
                    let stroke_w = if (stroke_width - 1.5).abs() > f32::EPSILON {
                        stroke_width.to_string()
                    } else {
                        sw.to_string()
                    };
                    match node.tag {
                        "path" => rsx! {
                            path {
                                d: node.d.unwrap_or(""),
                                stroke: "currentColor",
                                stroke_width: "{stroke_w}",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                fill: node.fill.unwrap_or("none"),
                            }
                        },
                        "circle" => rsx! {
                            circle {
                                cx: node.cx.unwrap_or("0"),
                                cy: node.cy.unwrap_or("0"),
                                r: node.r.unwrap_or("0"),
                                stroke: "currentColor",
                                stroke_width: "{stroke_w}",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                fill: node.fill.unwrap_or("none"),
                            }
                        },
                        "line" => rsx! {
                            line {
                                x1: node.x1.unwrap_or("0"),
                                y1: node.y1.unwrap_or("0"),
                                x2: node.x2.unwrap_or("0"),
                                y2: node.y2.unwrap_or("0"),
                                stroke: "currentColor",
                                stroke_width: "{stroke_w}",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                            }
                        },
                        "polyline" => rsx! {
                            polyline {
                                points: node.points.unwrap_or(""),
                                stroke: "currentColor",
                                stroke_width: "{stroke_w}",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                fill: "none",
                            }
                        },
                        "polygon" => rsx! {
                            polygon {
                                points: node.points.unwrap_or(""),
                                stroke: "currentColor",
                                stroke_width: "{stroke_w}",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                fill: node.fill.unwrap_or("none"),
                            }
                        },
                        "rect" => rsx! {
                            rect {
                                x: node.x.unwrap_or("0"),
                                y: node.y.unwrap_or("0"),
                                width: node.width.unwrap_or("0"),
                                height: node.height.unwrap_or("0"),
                                rx: node.r.unwrap_or("0"),
                                stroke: "currentColor",
                                stroke_width: "{stroke_w}",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                fill: node.fill.unwrap_or("none"),
                            }
                        },
                        _ => rsx! {},
                    }
                }
            }
        }
    }
}
`;

fs.writeFileSync("src/components/ui/hugeicon.rs", out);
console.log(
  `Generated ${Object.keys(selected).length} icons -> src/components/ui/hugeicon.rs`,
);
