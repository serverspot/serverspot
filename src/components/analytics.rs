use dioxus::prelude::*;

use crate::components::page::{
    DataPanel, FeatureSettingsChrome, PageHeader, RowItem, SettingRow, SettingsControl, StatusChip,
};
use crate::components::ui::*;
use crate::router::Route;

fn normalize_points(values: &[f32], width: f32, height: f32, pad: f32) -> Vec<(f32, f32)> {
    if values.is_empty() {
        return Vec::new();
    }
    let min = values.iter().copied().fold(f32::INFINITY, f32::min);
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let range = (max - min).max(0.0001);
    let steps = (values.len() - 1).max(1) as f32;
    let usable_h = (height - pad * 2.0).max(1.0);

    values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let x = width * (i as f32 / steps);
            let t = (v - min) / range;
            let y = pad + (1.0 - t) * usable_h;
            (x, y)
        })
        .collect()
}

fn line_path(points: &[(f32, f32)]) -> String {
    let mut d = String::new();
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            d.push_str(&format!("M{x:.1} {y:.1}"));
        } else {
            d.push_str(&format!(" L{x:.1} {y:.1}"));
        }
    }
    d
}

fn area_path(points: &[(f32, f32)], width: f32, height: f32) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut d = line_path(points);
    d.push_str(&format!(" L{width:.1} {height:.1} L0 {height:.1} Z"));
    d
}

fn polyline_length(points: &[(f32, f32)]) -> f32 {
    points
        .windows(2)
        .map(|w| {
            let (x0, y0) = w[0];
            let (x1, y1) = w[1];
            ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt()
        })
        .sum::<f32>()
        .max(1.0)
}

fn group_thousands(value: i64) -> String {
    let digits = value.abs().to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if value < 0 {
        format!("-{out}")
    } else {
        out
    }
}

fn format_metric(value: f32, prefix: &str, suffix: &str) -> String {
    let body = if value.abs() < 100.0 && value.fract().abs() > 0.001 {
        format!("{value:.1}")
    } else {
        group_thousands(value.round() as i64)
    };
    format!("{prefix}{body}{suffix}")
}

fn sparkline_svg(values: &[f32], color: &'static str, height: f32, class: &'static str) -> Element {
    let width = 120.0;
    let points = normalize_points(values, width, height, 3.0);
    let d = line_path(&points);
    let len = polyline_length(&points);

    rsx! {
        svg {
            class,
            view_box: "0 0 {width} {height}",
            preserve_aspect_ratio: "none",
            path {
                class: "chart-line-draw",
                style: "--path-len: {len:.1}",
                d,
                fill: "none",
                stroke: color,
                stroke_width: "1.6",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct ChartSeries {
    label: &'static str,
    color: &'static str,
    values: &'static [f32],
    fill: bool,
    prefix: &'static str,
    suffix: &'static str,
}

#[component]
fn ScopeChart(
    series: &'static [ChartSeries],
    labels: &'static [&'static str],
    #[props(default = 460.0)] width: f32,
    #[props(default = 150.0)] height: f32,
) -> Element {
    let mut hovered = use_signal(|| Option::<usize>::None);

    rsx! {
        div {
            class: "analytics-chart",
            onmouseleave: move |_| hovered.set(None),
            ScopeChartPlot { series, width, height }
            ScopeChartOverlay {
                series,
                width,
                height,
                hovered,
            }
            ScopeChartTip { series, labels, hovered }
        }
    }
}

#[component]
fn ScopeChartPlot(series: &'static [ChartSeries], width: f32, height: f32) -> Element {
    let grid_rows = 4;
    let paths = use_memo(move || {
        series
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let pts = normalize_points(s.values, width, height, 10.0);
                let line = line_path(&pts);
                let area = if s.fill {
                    area_path(&pts, width, height)
                } else {
                    String::new()
                };
                let len = polyline_length(&pts);
                (line, area, len, i)
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        svg { view_box: "0 0 {width} {height}", preserve_aspect_ratio: "none",
            for row in 0..=grid_rows {
                {
                    let y = height / grid_rows as f32 * row as f32;
                    rsx! {
                        line {
                            x1: "0",
                            x2: "{width}",
                            y1: "{y}",
                            y2: "{y}",
                            stroke: "var(--color-border-subtle)",
                            stroke_width: "1",
                        }
                    }
                }
            }

            for (line, area, len, i) in paths().into_iter() {
                {
                    let s = series[i];
                    let delay_fill = 0.1 + i as f32 * 0.1;
                    let delay_line = 0.04 + i as f32 * 0.12;
                    rsx! {
                        if s.fill {
                            path {
                                class: "chart-area-reveal",
                                style: "--draw-delay: {delay_fill:.2}s",
                                d: "{area}",
                                fill: "color-mix(in srgb, {s.color} 16%, transparent)",
                                stroke: "none",
                            }
                        }
                        path {
                            class: "chart-line-draw",
                            style: "--path-len: {len:.1}; --draw-delay: {delay_line:.2}s",
                            d: "{line}",
                            fill: "none",
                            stroke: s.color,
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScopeChartOverlay(
    series: &'static [ChartSeries],
    width: f32,
    height: f32,
    mut hovered: Signal<Option<usize>>,
) -> Element {
    let plotted: Vec<Vec<(f32, f32)>> = series
        .iter()
        .map(|s| normalize_points(s.values, width, height, 10.0))
        .collect();
    let point_count = plotted.iter().map(Vec::len).max().unwrap_or(0);
    let steps = point_count.saturating_sub(1).max(1) as f32;
    let band = width / point_count.max(1) as f32;
    let active = hovered().filter(|i| *i < point_count);

    rsx! {
        svg { view_box: "0 0 {width} {height}", preserve_aspect_ratio: "none",
            if let Some(idx) = active {
                {
                    let x = width * (idx as f32 / steps);
                    rsx! {
                        line {
                            class: "analytics-chart-crosshair",
                            x1: "{x}",
                            x2: "{x}",
                            y1: "0",
                            y2: "{height}",
                        }
                        for (i, s) in series.iter().enumerate() {
                            if let Some((cx, cy)) = plotted.get(i).and_then(|p| p.get(idx)).copied() {
                                circle {
                                    class: "analytics-chart-dot",
                                    cx: "{cx}",
                                    cy: "{cy}",
                                    r: "3.5",
                                    fill: s.color,
                                }
                            }
                        }
                    }
                }
            }
            for i in 0..point_count {
                {
                    let x = (width * (i as f32 / steps) - band / 2.0).max(0.0);
                    rsx! {
                        rect {
                            class: "analytics-chart-hit",
                            x: "{x}",
                            y: "0",
                            width: "{band}",
                            height: "{height}",
                            onmouseenter: move |_| hovered.set(Some(i)),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScopeChartTip(
    series: &'static [ChartSeries],
    labels: &'static [&'static str],
    hovered: Signal<Option<usize>>,
) -> Element {
    let point_count = series.iter().map(|s| s.values.len()).max().unwrap_or(0);
    let steps = point_count.saturating_sub(1).max(1) as f32;
    let Some(idx) = hovered().filter(|i| *i < point_count) else {
        return rsx! {};
    };
    let pct = 100.0 * idx as f32 / steps;
    let align = if pct < 18.0 {
        "analytics-tip is-start"
    } else if pct > 82.0 {
        "analytics-tip is-end"
    } else {
        "analytics-tip"
    };
    let heading = labels.get(idx).copied().unwrap_or("");

    rsx! {
        div { class: align, style: "--tip-x: {pct:.2}%",
            if !heading.is_empty() {
                p { class: "analytics-tip-label", "{heading}" }
            }
            for s in series.iter() {
                if let Some(v) = s.values.get(idx).copied() {
                    div { class: "analytics-tip-row",
                        span { class: "analytics-tip-key",
                            span {
                                class: "analytics-tip-swatch",
                                style: "--dot-color: {s.color}",
                            }
                            "{s.label}"
                        }
                        span { class: "analytics-tip-value", {format_metric(v, s.prefix, s.suffix)} }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Trend {
    Up,
    Down,
    Flat,
}

impl Trend {
    fn class(self) -> &'static str {
        match self {
            Trend::Up => "analytics-delta is-up",
            Trend::Down => "analytics-delta is-down",
            Trend::Flat => "analytics-delta is-flat",
        }
    }
}

#[component]
fn MetricCard(
    label: &'static str,
    value: &'static str,
    delta: &'static str,
    trend: Trend,
    #[props(default = "#38bdf8")] color: &'static str,
    points: &'static [f32],
) -> Element {
    rsx! {
        div { class: "analytics-metric",
            p { class: "analytics-metric-label", "{label}" }
            div { class: "analytics-metric-value-row",
                p { class: "analytics-metric-value", "{value}" }
                span { class: trend.class(), "{delta}" }
            }
            {sparkline_svg(points, color, 34.0, "analytics-metric-spark")}
        }
    }
}

#[component]
fn DrillInRow(
    title: &'static str,
    meta: &'static str,
    trailing: &'static str,
    to: Route,
) -> Element {
    let navigator = use_navigator();
    let dest = to;

    rsx! {
        button {
            class: "attention-row",
            onclick: move |_| {
                navigator.push(dest.clone());
            },
            div { class: "min-w-0",
                p { class: "text-sm font-medium text-text-secondary", "{title}" }
                p { class: "mt-0.5 text-xs text-text-muted", "{meta}" }
            }
            span { class: "shrink-0 text-xs font-semibold text-accent", "{trailing}" }
        }
    }
}

const VISITORS_SERIES: &[f32] = &[
    510., 540., 498., 610., 655., 700., 760., 690., 810., 845., 790., 860., 902., 842.,
];
const REVENUE_SERIES: &[f32] = &[
    62., 58., 70., 64., 80., 76., 92., 88., 95., 101., 97., 108., 104., 112.,
];
const PLAYERS_SERIES: &[f32] = &[
    140., 165., 158., 190., 172., 205., 230., 198., 245., 260., 232., 268., 275., 254.,
];
const CONVERSION_SERIES: &[f32] = &[
    2.9, 3.1, 3.0, 3.4, 3.2, 3.6, 3.5, 3.7, 3.8, 3.6, 3.9, 3.8, 4.0, 3.8,
];
const CSAT_SERIES: &[f32] = &[
    90., 91., 89., 92., 93., 91., 94., 93., 95., 94., 93., 95., 94., 94.,
];

const OVERVIEW_DAYS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

const OVERVIEW_VISITORS_WEEK: &[f32] = &[690., 810., 845., 790., 860., 902., 842.];
const OVERVIEW_PLAYERS_WEEK: &[f32] = &[198., 245., 260., 232., 268., 275., 254.];
const OVERVIEW_REVENUE_WEEK: &[f32] = &[88., 95., 101., 97., 108., 104., 112.];

const OVERVIEW_COMPOSITE: &[ChartSeries] = &[
    ChartSeries {
        label: "Visitors",
        color: "#38bdf8",
        values: OVERVIEW_VISITORS_WEEK,
        fill: true,
        prefix: "",
        suffix: "",
    },
    ChartSeries {
        label: "Players",
        color: "#5eead4",
        values: OVERVIEW_PLAYERS_WEEK,
        fill: true,
        prefix: "",
        suffix: "",
    },
    ChartSeries {
        label: "Revenue",
        color: "#fbbf24",
        values: OVERVIEW_REVENUE_WEEK,
        fill: false,
        prefix: "£",
        suffix: "",
    },
];

const CHANNELS: &[(&str, f32, &str, Route)] = &[
    ("Website", 82.0, "6,104 users", Route::AnalyticsWebsite {}),
    (
        "Community",
        64.0,
        "890 active members",
        Route::AnalyticsCommunity {},
    ),
    (
        "Gaming",
        91.0,
        "412 peak players",
        Route::AnalyticsGaming {},
    ),
];

#[component]
pub fn AnalyticsOverview() -> Element {
    let navigator = use_navigator();

    rsx! {
        div { class: "analytics-console-masthead",
            div { class: "min-w-0",
                p { class: "analytics-console-eyebrow", "Analytics" }
                h1 { class: "analytics-console-title", "8,420 visitors this week" }
                p { class: "analytics-console-sub",
                    "£1,094 revenue · 3.8% conversion · 412 peak players"
                }
            }
            div { class: "flex flex-wrap items-center gap-2",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::AnalyticsSiteSettings {});
                    },
                    "Retention & privacy"
                }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::AnalyticsWebsite {});
                    },
                    "Website report"
                }
            }
        }

        section { class: "motion-cascade analytics-metrics mb-4",
            MetricCard {
                label: "Revenue",
                value: "£1,094",
                delta: "▲ 8.2%",
                trend: Trend::Up,
                color: "#fbbf24",
                points: REVENUE_SERIES,
            }
            MetricCard {
                label: "Visitors",
                value: "8,420",
                delta: "▲ 4.6%",
                trend: Trend::Up,
                color: "#38bdf8",
                points: VISITORS_SERIES,
            }
            MetricCard {
                label: "Conversion",
                value: "3.8%",
                delta: "▼ 0.2pt",
                trend: Trend::Down,
                color: "#f071a5",
                points: CONVERSION_SERIES,
            }
            MetricCard {
                label: "Ticket CSAT",
                value: "94%",
                delta: "flat",
                trend: Trend::Flat,
                color: "#3ecf8e",
                points: CSAT_SERIES,
            }
        }

        div { class: "mb-4",
            DataPanel { title: "Visitors, players, and revenue — last 7 days",
                div { class: "analytics-chart-legend",
                    span { class: "analytics-chart-legend-item",
                        span {
                            class: "analytics-chart-legend-dot",
                            style: "--dot-color: #38bdf8;",
                        }
                        "Visitors"
                    }
                    span { class: "analytics-chart-legend-item",
                        span {
                            class: "analytics-chart-legend-dot",
                            style: "--dot-color: #5eead4;",
                        }
                        "Players"
                    }
                    span { class: "analytics-chart-legend-item",
                        span {
                            class: "analytics-chart-legend-dot",
                            style: "--dot-color: #fbbf24;",
                        }
                        "Revenue"
                    }
                }
                ScopeChart {
                    series: OVERVIEW_COMPOSITE,
                    labels: OVERVIEW_DAYS,
                    width: 460.0,
                    height: 150.0,
                }
                div { class: "analytics-chart-axis",
                    for day in OVERVIEW_DAYS.iter() {
                        span { "{day}" }
                    }
                }
            }
        }

        div { class: "motion-cascade grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Channels",
                for (name, value, meta, to) in CHANNELS.iter().cloned() {
                    ChannelRow {
                        name,
                        value,
                        meta,
                        to,
                    }
                }
            }
            DataPanel { title: "Reports",
                DrillInRow {
                    title: "Website",
                    meta: "Traffic, sources, and pages",
                    trailing: "Open",
                    to: Route::AnalyticsWebsite {},
                }
                DrillInRow {
                    title: "Community",
                    meta: "Registrations and engagement",
                    trailing: "Open",
                    to: Route::AnalyticsCommunity {},
                }
                DrillInRow {
                    title: "Gaming",
                    meta: "Player counts and server health",
                    trailing: "Open",
                    to: Route::AnalyticsGaming {},
                }
                DrillInRow {
                    title: "Retention & privacy",
                    meta: "Configure data collection",
                    trailing: "Open",
                    to: Route::AnalyticsSiteSettings {},
                }
            }
        }
    }
}

#[component]
fn ChannelRow(name: &'static str, value: f32, meta: &'static str, to: Route) -> Element {
    let navigator = use_navigator();
    let dest = to;
    let pct = value.clamp(0.0, 100.0);

    rsx! {
        button {
            r#type: "button",
            class: "analytics-row is-clickable",
            onclick: move |_| {
                navigator.push(dest.clone());
            },
            div { class: "analytics-row-main",
                p { class: "analytics-row-label", "{name}" }
                p { class: "analytics-row-meta", "{meta}" }
                div { class: "analytics-bar-track",
                    div { class: "analytics-bar-fill", style: "width: {pct}%;" }
                }
            }
            span { class: "analytics-row-value", "{pct}%" }
        }
    }
}

const WEEKLY_TRAFFIC: &[f32] = &[420., 460., 402., 512., 470., 605., 560.];
const TRAFFIC_DAYS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const WEBSITE_TRAFFIC_SERIES: &[ChartSeries] = &[ChartSeries {
    label: "Visits",
    color: "#38bdf8",
    values: WEEKLY_TRAFFIC,
    fill: true,
    prefix: "",
    suffix: "",
}];

const TRAFFIC_SOURCES: &[(&str, &str, f32)] = &[
    ("Direct", "Returning players", 38.0),
    ("Discord", "Invite & announcements", 27.0),
    ("Search", "Organic", 18.0),
    ("Vote sites", "Reward campaigns", 17.0),
];

const POPULAR_PAGES: &[(&str, &str, &[f32])] = &[
    (
        "/store",
        "4.2k views this week",
        &[520., 560., 540., 610., 640., 700., 720.],
    ),
    (
        "/help/vote-rewards",
        "2.1k views this week",
        &[300., 280., 320., 310., 350., 330., 360.],
    ),
    (
        "/leaderboards",
        "1.4k views this week",
        &[180., 200., 190., 210., 205., 240., 250.],
    ),
];

#[component]
pub fn AnalyticsWebsite() -> Element {
    rsx! {
        PageHeader {
            title: "Website analytics",
            subtitle: "Users, page views, traffic sources, and popular pages.",
        }

        section { class: "motion-cascade analytics-metrics mb-4",
            MetricCard {
                label: "Users",
                value: "6,104",
                delta: "▲ 5.1%",
                trend: Trend::Up,
                color: "#38bdf8",
                points: VISITORS_SERIES,
            }
            MetricCard {
                label: "Page views",
                value: "28.4k",
                delta: "▲ 2.4%",
                trend: Trend::Up,
                color: "#87d1fe",
                points: PLAYERS_SERIES,
            }
            MetricCard {
                label: "Bounce rate",
                value: "41%",
                delta: "▲ 1.1pt",
                trend: Trend::Down,
                color: "#f0a35e",
                points: CONVERSION_SERIES,
            }
            MetricCard {
                label: "Avg. session",
                value: "3m 12s",
                delta: "flat",
                trend: Trend::Flat,
                color: "#3ecf8e",
                points: CSAT_SERIES,
            }
        }

        div { class: "motion-cascade grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Weekly traffic",
                ScopeChart {
                    series: WEBSITE_TRAFFIC_SERIES,
                    labels: TRAFFIC_DAYS,
                    width: 400.0,
                    height: 140.0,
                }
                div { class: "analytics-chart-axis",
                    for day in TRAFFIC_DAYS.iter() {
                        span { "{day}" }
                    }
                }
            }
            DataPanel { title: "Traffic sources",
                for (name, meta, share) in TRAFFIC_SOURCES.iter().copied() {
                    div { class: "analytics-row",
                        div { class: "analytics-row-main",
                            p { class: "analytics-row-label", "{name}" }
                            p { class: "analytics-row-meta", "{meta}" }
                            div { class: "analytics-bar-track",
                                div {
                                    class: "analytics-bar-fill",
                                    style: "width: {share}%;",
                                }
                            }
                        }
                        span { class: "analytics-row-value", "{share}%" }
                    }
                }
            }
        }

        div { class: "mt-4",
            DataPanel { title: "Popular pages",
                for (path, meta, series) in POPULAR_PAGES.iter().copied() {
                    div { class: "analytics-row",
                        div { class: "analytics-row-main",
                            p { class: "analytics-row-label is-path", "{path}" }
                            p { class: "analytics-row-meta", "{meta}" }
                        }
                        {sparkline_svg(series, "#38bdf8", 26.0, "analytics-row-spark")}
                    }
                }
            }
        }
    }
}

const CHANNEL_ENGAGEMENT: &[(&str, &str, f32, &[f32])] = &[
    (
        "Forum",
        "#5b9dff",
        78.0,
        &[40., 65., 50., 80., 60., 90., 70.],
    ),
    (
        "Support",
        "#f0a35e",
        52.0,
        &[55., 45., 60., 50., 40., 58., 52.],
    ),
    (
        "Blog",
        "#f071a5",
        61.0,
        &[30., 42., 38., 55., 48., 60., 61.],
    ),
];

#[component]
pub fn AnalyticsCommunity() -> Element {
    rsx! {
        PageHeader {
            title: "Community analytics",
            subtitle: "Forum activity, registrations, and cross-channel engagement.",
        }

        section { class: "motion-cascade analytics-metrics mb-4",
            MetricCard {
                label: "Registrations",
                value: "146",
                delta: "▲ 12",
                trend: Trend::Up,
                color: "#3ecf8e",
                points: PLAYERS_SERIES,
            }
            MetricCard {
                label: "Forum posts",
                value: "512",
                delta: "▲ 38",
                trend: Trend::Up,
                color: "#f071a5",
                points: VISITORS_SERIES,
            }
            MetricCard {
                label: "Active members",
                value: "890",
                delta: "▲ 3.2%",
                trend: Trend::Up,
                color: "#5b9dff",
                points: REVENUE_SERIES,
            }
            MetricCard {
                label: "Engagement",
                value: "64%",
                delta: "flat",
                trend: Trend::Flat,
                color: "#87d1fe",
                points: CSAT_SERIES,
            }
        }

        div { class: "motion-cascade grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Engagement by channel",
                for (label, color, value, series) in CHANNEL_ENGAGEMENT.iter().copied() {
                    div { class: "analytics-row",
                        div { class: "analytics-row-main",
                            p { class: "analytics-row-label", "{label}" }
                            p { class: "analytics-row-meta", "Last 7 days" }
                            div { class: "analytics-bar-track",
                                div {
                                    class: "analytics-bar-fill",
                                    style: "width: {value}%; --bar-color: {color};",
                                }
                            }
                        }
                        {sparkline_svg(series, color, 26.0, "analytics-row-spark")}
                        span { class: "analytics-row-value", "{value}%" }
                    }
                }
            }
            DataPanel { title: "This week",
                RowItem {
                    title: "New forum threads",
                    meta: "Survival and Suggestions leading",
                    trailing: "+38",
                }
                RowItem {
                    title: "Verified emails",
                    meta: "Registration funnel",
                    trailing: "91%",
                }
                RowItem {
                    title: "Returning visitors",
                    meta: "7-day window",
                    trailing: "54%",
                }
                RowItem {
                    title: "Application starts",
                    meta: "Staff recruitment",
                    trailing: "11",
                }
            }
        }
    }
}

struct ServerTelemetry {
    name: &'static str,
    status: &'static str,
    status_tone: &'static str,
    online: &'static str,
    uptime: &'static str,
    load: &'static str,
    pulse: &'static [f32],
    color: &'static str,
}

const SERVERS: &[ServerTelemetry] = &[
    ServerTelemetry {
        name: "Survival",
        status: "Healthy",
        status_tone: "#3ecf8e",
        online: "186 avg",
        uptime: "99.9%",
        load: "Economy climbing",
        pulse: &[10., 12., 40., 12., 10., 55., 14., 10., 38., 12.],
        color: "#3ecf8e",
    },
    ServerTelemetry {
        name: "Skyblock",
        status: "Healthy",
        status_tone: "#3ecf8e",
        online: "94 avg",
        uptime: "99.7%",
        load: "Vote claims up",
        pulse: &[12., 42., 12., 10., 48., 12., 10., 44., 12., 10.],
        color: "#5eead4",
    },
    ServerTelemetry {
        name: "Creative",
        status: "Steady",
        status_tone: "#38bdf8",
        online: "41 avg",
        uptime: "99.4%",
        load: "Builder spike weekends",
        pulse: &[8., 10., 30., 10., 8., 32., 10., 8., 28., 10.],
        color: "#38bdf8",
    },
    ServerTelemetry {
        name: "Lobby",
        status: "OK",
        status_tone: "#fbbf24",
        online: "Queue normal",
        uptime: "99.8%",
        load: "No incidents",
        pulse: &[14., 16., 18., 15., 17., 16., 18., 15., 17., 16.],
        color: "#fbbf24",
    },
];

#[component]
pub fn AnalyticsGaming() -> Element {
    rsx! {
        PageHeader {
            title: "Gaming analytics",
            subtitle: "Player counts, votes, leaderboards, and server telemetry.",
        }

        section { class: "motion-cascade analytics-metrics mb-4",
            MetricCard {
                label: "Peak players",
                value: "412",
                delta: "▲ 6.4%",
                trend: Trend::Up,
                color: "#5eead4",
                points: PLAYERS_SERIES,
            }
            MetricCard {
                label: "Votes",
                value: "1,480",
                delta: "▲ 9.8%",
                trend: Trend::Up,
                color: "#fbbf24",
                points: REVENUE_SERIES,
            }
            MetricCard {
                label: "Leaderboard hits",
                value: "9.2k",
                delta: "▲ 3.0%",
                trend: Trend::Up,
                color: "#38bdf8",
                points: VISITORS_SERIES,
            }
            MetricCard {
                label: "Server uptime",
                value: "99.8%",
                delta: "flat",
                trend: Trend::Flat,
                color: "#3ecf8e",
                points: CSAT_SERIES,
            }
        }

        section { class: "analytics-panel",
            div { class: "analytics-panel-head",
                h2 { class: "text-sm font-semibold text-text", "Servers" }
                span { class: "text-xs text-text-muted", "Averages over the last 7 days" }
            }
            div { class: "motion-cascade motion-cascade-tight analytics-table",
                div { class: "analytics-table-row is-head",
                    span { "Server" }
                    span { "Status" }
                    span { "Online" }
                    span { "Uptime" }
                    span { "Trend" }
                    span { "Note" }
                }
                for server in SERVERS.iter() {
                    div { key: "{server.name}", class: "analytics-table-row",
                        span { class: "analytics-table-name", "{server.name}" }
                        span {
                            StatusChip {
                                label: server.status,
                                tone: server.status_tone,
                            }
                        }
                        span { "{server.online}" }
                        span { class: "tabular-nums", "{server.uptime}" }
                        span { {sparkline_svg(server.pulse, server.color, 24.0, "analytics-row-spark")} }
                        span { class: "text-text-muted", "{server.load}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AnalyticsSiteSettings() -> Element {
    let retention_value = use_signal(|| "90".to_string());
    let retention_days: u32 = retention_value().parse().unwrap_or(90);

    let options = vec![
        SelectOption::new("30", "30 days"),
        SelectOption::new("90", "90 days"),
        SelectOption::new("180", "180 days"),
        SelectOption::new("365", "365 days"),
    ];

    rsx! {
        FeatureSettingsChrome { subtitle: "Control how long raw analytics events are kept and how visitors are tracked.",
            DataPanel { title: "Data retention window",
                p { class: "py-3 text-sm text-text-muted",
                    "Raw event data older than the window below is automatically purged. Aggregated readouts are kept indefinitely."
                }
                SettingsControl { label: "Retention window",
                    SignalSelect {
                        value: retention_value,
                        options,
                        placeholder: "Retention window".to_string(),
                    }
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Currently retaining raw events for {retention_days} days · applies to website, community, and gaming data."
                }
            }
            DataPanel { title: "Collection & privacy",
                SettingRow {
                    title: "Anonymize IP addresses",
                    description: "Truncate the last octet before any event is stored.",
                    enabled: true,
                }
                SettingRow {
                    title: "Respect Do Not Track",
                    description: "Skip collection entirely for visitors sending DNT signals.",
                    enabled: true,
                }
                SettingRow {
                    title: "Cookieless session tracking",
                    description: "Use a rotating server-side session key instead of cookies.",
                    enabled: false,
                }
                SettingRow {
                    title: "Include in GDPR exports",
                    description: "Bundle analytics events with player data export requests.",
                    enabled: true,
                }
            }
            DataPanel { title: "Recent data requests",
                RowItem {
                    title: "Export request · NovaCraft",
                    meta: "GDPR data export · Fulfilled",
                    trailing: "2d ago",
                }
                RowItem {
                    title: "Deletion request · QuietLeaf",
                    meta: "Right to erasure · Fulfilled",
                    trailing: "5d ago",
                }
                RowItem {
                    title: "Retention sweep",
                    meta: "Purged events older than 90 days",
                    trailing: "Nightly",
                }
            }
        }
    }
}
