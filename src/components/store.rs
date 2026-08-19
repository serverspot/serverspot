use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::page::{
    DataPanel, FeatureSettingsChrome, SettingRow, SettingsControl, SettingsField, StatusChip,
};
use crate::components::ui::*;
use crate::i18n::t_key;
use crate::router::Route;

const STORE_ACCENT: &str = "#3ecf8e";
const WEEK_REVENUE: &[u32] = &[78, 112, 94, 156, 128, 142, 102];
const WEEKDAY_KEYS: &[&str] = &[
    "store-weekday-mon",
    "store-weekday-tue",
    "store-weekday-wed",
    "store-weekday-thu",
    "store-weekday-fri",
    "store-weekday-sat",
    "store-weekday-sun",
];
const WEEKDAY_FULL_KEYS: &[&str] = &[
    "store-weekday-monday",
    "store-weekday-tuesday",
    "store-weekday-wednesday",
    "store-weekday-thursday",
    "store-weekday-friday",
    "store-weekday-saturday",
    "store-weekday-sunday",
];
const ALL_SERVERS_KEY: &str = "all-servers";

#[derive(Clone, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price: String,
    pub sale_price: String,
    pub stock: String,
    pub visible: bool,
    pub featured: bool,
    pub giftable: bool,
    pub require_online: bool,
    pub servers: Vec<String>,
    pub commands: String,
    pub revoke_commands: String,
    pub expiry_days: String,
    pub sold: u32,
    pub media: Vec<String>,
}

impl Product {
    fn blank() -> Self {
        Self {
            id: 0,
            name: String::new(),
            slug: String::new(),
            description: String::new(),
            category: String::from("Uncategorised"),
            price: String::from("£0.00"),
            sale_price: String::new(),
            stock: String::from("Unlimited"),
            visible: true,
            featured: false,
            giftable: true,
            require_online: false,
            servers: vec![String::from("Survival"), String::from("Skyblock")],
            commands: String::new(),
            revoke_commands: String::new(),
            expiry_days: String::new(),
            sold: 0,
            media: vec![],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OrderStatus {
    All,
    Paid,
    Pending,
    Refunded,
}

impl OrderStatus {
    fn label(self) -> String {
        match self {
            Self::All => t_key("store-order-status-all"),
            Self::Paid => t_key("store-order-status-paid"),
            Self::Pending => t_key("store-order-status-pending"),
            Self::Refunded => t_key("store-order-status-refunded"),
        }
    }

    fn tone(self) -> &'static str {
        match self {
            Self::All => "#a1a1aa",
            Self::Paid => "#3ecf8e",
            Self::Pending => "#fbbf24",
            Self::Refunded => "#f87171",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PaymentFilter {
    All,
    Stripe,
    PayPal,
}

impl PaymentFilter {
    fn store_key(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Stripe => "stripe",
            Self::PayPal => "paypal",
        }
    }

    fn from_store_key(key: &str) -> Self {
        match key {
            "stripe" => Self::Stripe,
            "paypal" => Self::PayPal,
            _ => Self::All,
        }
    }

    fn label(self) -> String {
        match self {
            Self::All => t_key("store-payment-filter-all"),
            Self::Stripe => t_key("store-payment-filter-stripe"),
            Self::PayPal => t_key("store-payment-filter-paypal"),
        }
    }

    fn matches(self, method: &str) -> bool {
        match self {
            Self::All => true,
            Self::Stripe => method.to_lowercase().contains("stripe"),
            Self::PayPal => method.to_lowercase().contains("paypal"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DeliveryFilter {
    All,
    Delivered,
    Queued,
    Issue,
}

impl DeliveryFilter {
    fn store_key(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Delivered => "delivered",
            Self::Queued => "queued",
            Self::Issue => "issue",
        }
    }

    fn from_store_key(key: &str) -> Self {
        match key {
            "delivered" => Self::Delivered,
            "queued" => Self::Queued,
            "issue" => Self::Issue,
            _ => Self::All,
        }
    }

    fn label(self) -> String {
        match self {
            Self::All => t_key("store-delivery-filter-all"),
            Self::Delivered => t_key("store-delivery-filter-delivered"),
            Self::Queued => t_key("store-delivery-filter-queued"),
            Self::Issue => t_key("store-delivery-filter-issue"),
        }
    }

    fn matches(self, delivery: &str) -> bool {
        let lower = delivery.to_lowercase();
        match self {
            Self::All => true,
            Self::Delivered => lower.contains("delivered"),
            Self::Queued => lower.contains("queued") || lower.contains("waiting"),
            Self::Issue => lower.contains("revoked") || lower.contains("failed"),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Order {
    id: u64,
    product: String,
    player: String,
    email: String,
    status: OrderStatus,
    amount: String,
    when: String,
    method: String,
    delivery: String,
    server: String,
}

pub(crate) fn placeholder_products() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: String::from("VIP Rank"),
            slug: String::from("vip-rank"),
            description: String::from("Priority queue, /fly, and exclusive kit access."),
            category: String::from("Ranks"),
            price: String::from("£29.99"),
            sale_price: String::new(),
            stock: String::from("Unlimited"),
            visible: true,
            featured: true,
            giftable: true,
            require_online: false,
            servers: vec![String::from("Survival"), String::from("Skyblock")],
            commands: String::from("lp user {player} parent add vip"),
            revoke_commands: String::from("lp user {player} parent remove vip"),
            expiry_days: String::from("30"),
            sold: 42,
            media: vec![],
        },
        Product {
            id: 2,
            name: String::from("Crate Key Bundle"),
            slug: String::from("crate-key-bundle"),
            description: String::from("Five legendary crate keys delivered on purchase."),
            category: String::from("Crates"),
            price: String::from("£9.99"),
            sale_price: String::from("£7.99"),
            stock: String::from("240"),
            visible: true,
            featured: false,
            giftable: true,
            require_online: true,
            servers: vec![String::from("Survival")],
            commands: String::from("crates give {player} legendary 5"),
            revoke_commands: String::new(),
            expiry_days: String::new(),
            sold: 31,
            media: vec![],
        },
        Product {
            id: 3,
            name: String::from("Cosmetics Pack"),
            slug: String::from("cosmetics-pack"),
            description: String::from("Season 4 particle trails and hat cosmetics."),
            category: String::from("Cosmetics"),
            price: String::from("£14.99"),
            sale_price: String::new(),
            stock: String::from("4"),
            visible: true,
            featured: false,
            giftable: true,
            require_online: false,
            servers: vec![String::from("Lobby"), String::from("Survival")],
            commands: String::from("cosmetics unlock {player} season4"),
            revoke_commands: String::from("cosmetics lock {player} season4"),
            expiry_days: String::new(),
            sold: 19,
            media: vec![],
        },
        Product {
            id: 4,
            name: String::from("Home Teleport"),
            slug: String::from("home-teleport"),
            description: String::from("Unlock an extra /sethome slot."),
            category: String::from("Perks"),
            price: String::from("£4.99"),
            sale_price: String::new(),
            stock: String::from("Unlimited"),
            visible: true,
            featured: false,
            giftable: false,
            require_online: true,
            servers: vec![String::from("Survival"), String::from("Skyblock")],
            commands: String::from("essentials:sethome {player}"),
            revoke_commands: String::new(),
            expiry_days: String::new(),
            sold: 14,
            media: vec![],
        },
        Product {
            id: 5,
            name: String::from("MVP Rank"),
            slug: String::from("mvp-rank"),
            description: String::from("Everything in VIP plus custom prefix and double XP."),
            category: String::from("Ranks"),
            price: String::from("£49.99"),
            sale_price: String::new(),
            stock: String::from("Unlimited"),
            visible: false,
            featured: false,
            giftable: true,
            require_online: false,
            servers: vec![
                String::from("Survival"),
                String::from("Skyblock"),
                String::from("Creative"),
            ],
            commands: String::from("lp user {player} parent add mvp"),
            revoke_commands: String::from("lp user {player} parent remove mvp"),
            expiry_days: String::from("30"),
            sold: 7,
            media: vec![],
        },
        Product {
            id: 6,
            name: String::from("Starter Bundle"),
            slug: String::from("starter-bundle"),
            description: String::from("Kit, keys, and a welcome rank for new players."),
            category: String::from("Bundles"),
            price: String::from("£19.99"),
            sale_price: String::from("£14.99"),
            stock: String::from("86"),
            visible: true,
            featured: true,
            giftable: true,
            require_online: true,
            servers: vec![String::from("Survival")],
            commands: String::from("kit give {player} starter"),
            revoke_commands: String::new(),
            expiry_days: String::new(),
            sold: 22,
            media: vec![],
        },
    ]
}

const ORDER_PAGE_SIZE: usize = 6;

fn placeholder_orders() -> Vec<Order> {
    vec![
        Order {
            id: 4821,
            product: String::from("VIP Rank"),
            player: String::from("NovaCraft"),
            email: String::from("novacraft@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£29.99"),
            when: String::from("2m"),
            method: String::from("Stripe · Visa"),
            delivery: String::from("Delivered"),
            server: String::from("Survival"),
        },
        Order {
            id: 4818,
            product: String::from("Crate Key Bundle"),
            player: String::from("SkyBuilder"),
            email: String::from("skybuilder@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£9.99"),
            when: String::from("18m"),
            method: String::from("PayPal"),
            delivery: String::from("Delivered"),
            server: String::from("Skyblock"),
        },
        Order {
            id: 4812,
            product: String::from("Cosmetics Pack"),
            player: String::from("AetherFox"),
            email: String::from("aetherfox@players.serverspot.app"),
            status: OrderStatus::Refunded,
            amount: String::from("£14.99"),
            when: String::from("1h"),
            method: String::from("Stripe · Mastercard"),
            delivery: String::from("Revoked"),
            server: String::from("Lobby"),
        },
        Order {
            id: 4809,
            product: String::from("Home Teleport"),
            player: String::from("RedstoneRex"),
            email: String::from("redstonerex@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£4.99"),
            when: String::from("3h"),
            method: String::from("PayPal"),
            delivery: String::from("Queued"),
            server: String::from("Lobby"),
        },
        Order {
            id: 4801,
            product: String::from("Starter Bundle"),
            player: String::from("QuietLeaf"),
            email: String::from("quietleaf@players.serverspot.app"),
            status: OrderStatus::Pending,
            amount: String::from("£19.99"),
            when: String::from("5h"),
            method: String::from("Stripe · awaiting 3DS"),
            delivery: String::from("Waiting for payment"),
            server: String::from("Survival"),
        },
        Order {
            id: 4794,
            product: String::from("MVP Rank"),
            player: String::from("CopperFox"),
            email: String::from("copperfox@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£49.99"),
            when: String::from("Yesterday"),
            method: String::from("Stripe · Apple Pay"),
            delivery: String::from("Delivered"),
            server: String::from("Creative"),
        },
        Order {
            id: 4788,
            product: String::from("Crate Key Bundle"),
            player: String::from("MossyBee"),
            email: String::from("mossybee@players.serverspot.app"),
            status: OrderStatus::Pending,
            amount: String::from("£9.99"),
            when: String::from("Yesterday"),
            method: String::from("PayPal"),
            delivery: String::from("Waiting for payment"),
            server: String::from("Survival"),
        },
        Order {
            id: 4772,
            product: String::from("VIP Rank"),
            player: String::from("PixelNomad"),
            email: String::from("pixelnomad@players.serverspot.app"),
            status: OrderStatus::Refunded,
            amount: String::from("£29.99"),
            when: String::from("2d"),
            method: String::from("Stripe · Visa"),
            delivery: String::from("Revoked"),
            server: String::from("Skyblock"),
        },
        Order {
            id: 4765,
            product: String::from("Cosmetics Pack"),
            player: String::from("LunarWisp"),
            email: String::from("lunarwisp@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£14.99"),
            when: String::from("2d"),
            method: String::from("Stripe · Google Pay"),
            delivery: String::from("Delivered"),
            server: String::from("Lobby"),
        },
        Order {
            id: 4758,
            product: String::from("Starter Bundle"),
            player: String::from("OakSpire"),
            email: String::from("oakspire@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£19.99"),
            when: String::from("3d"),
            method: String::from("PayPal"),
            delivery: String::from("Delivered"),
            server: String::from("Survival"),
        },
        Order {
            id: 4751,
            product: String::from("Home Teleport"),
            player: String::from("GlintFox"),
            email: String::from("glintfox@players.serverspot.app"),
            status: OrderStatus::Pending,
            amount: String::from("£4.99"),
            when: String::from("3d"),
            method: String::from("Stripe · Visa"),
            delivery: String::from("Waiting for payment"),
            server: String::from("Creative"),
        },
        Order {
            id: 4744,
            product: String::from("MVP Rank"),
            player: String::from("TideRunner"),
            email: String::from("tiderunner@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£49.99"),
            when: String::from("4d"),
            method: String::from("Stripe · Mastercard"),
            delivery: String::from("Delivered"),
            server: String::from("Skyblock"),
        },
        Order {
            id: 4737,
            product: String::from("Crate Key Bundle"),
            player: String::from("AshTrail"),
            email: String::from("ashtrail@players.serverspot.app"),
            status: OrderStatus::Refunded,
            amount: String::from("£9.99"),
            when: String::from("5d"),
            method: String::from("PayPal"),
            delivery: String::from("Revoked"),
            server: String::from("Survival"),
        },
        Order {
            id: 4730,
            product: String::from("VIP Rank"),
            player: String::from("CopperBloom"),
            email: String::from("copperbloom@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£29.99"),
            when: String::from("5d"),
            method: String::from("Stripe · Apple Pay"),
            delivery: String::from("Queued"),
            server: String::from("Lobby"),
        },
        Order {
            id: 4722,
            product: String::from("Cosmetics Pack"),
            player: String::from("NestByte"),
            email: String::from("nestbyte@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£14.99"),
            when: String::from("6d"),
            method: String::from("Stripe · Visa"),
            delivery: String::from("Delivered"),
            server: String::from("Creative"),
        },
        Order {
            id: 4715,
            product: String::from("Starter Bundle"),
            player: String::from("PaleForge"),
            email: String::from("paleforge@players.serverspot.app"),
            status: OrderStatus::Pending,
            amount: String::from("£19.99"),
            when: String::from("1w"),
            method: String::from("PayPal"),
            delivery: String::from("Waiting for payment"),
            server: String::from("Skyblock"),
        },
        Order {
            id: 4708,
            product: String::from("Home Teleport"),
            player: String::from("RiverKnot"),
            email: String::from("riverknot@players.serverspot.app"),
            status: OrderStatus::Paid,
            amount: String::from("£4.99"),
            when: String::from("1w"),
            method: String::from("Stripe · Google Pay"),
            delivery: String::from("Delivered"),
            server: String::from("Survival"),
        },
        Order {
            id: 4701,
            product: String::from("MVP Rank"),
            player: String::from("EmberCast"),
            email: String::from("embercast@players.serverspot.app"),
            status: OrderStatus::Refunded,
            amount: String::from("£49.99"),
            when: String::from("1w"),
            method: String::from("Stripe · Mastercard"),
            delivery: String::from("Revoked"),
            server: String::from("Lobby"),
        },
    ]
}

#[component]
fn StoreFormField(#[props(into)] label: String, children: Element) -> Element {
    let _lang = i18n();
    rsx! {
        label { class: "block",
            span { class: "mb-1.5 block text-xs font-medium text-text-secondary", "{label}" }
            {children}
        }
    }
}

#[component]
fn PaymentFilterSelect(mut value: Signal<PaymentFilter>) -> Element {
    let _lang = i18n();
    let mut open = use_signal(|| false);
    let options = [
        PaymentFilter::All,
        PaymentFilter::Stripe,
        PaymentFilter::PayPal,
    ];
    let display = value().label();
    let is_open = open();

    rsx! {
        div {
            class: if is_open { "ui-dropdown is-open" } else { "ui-dropdown" },
            button {
                r#type: "button",
                class: if is_open { "ui-dropdown-trigger ui-squircle is-open" } else { "ui-dropdown-trigger ui-squircle" },
                "aria-haspopup": "listbox",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |_| {
                    let next = !open();
                    open.set(next);
                },
                span { class: "ui-dropdown-value", "{display}" }
                span { class: "ui-dropdown-caret", "aria-hidden": "true" }
            }
            if is_open {
                div { class: "ui-dropdown-menu ui-squircle", role: "listbox",
                    for filter in options {
                        {
                            let selected = value() == filter;
                            rsx! {
                                button {
                                    key: "{filter.store_key()}",
                                    r#type: "button",
                                    class: if selected { "ui-dropdown-option is-selected" } else { "ui-dropdown-option" },
                                    role: "option",
                                    "aria-selected": if selected { "true" } else { "false" },
                                    onclick: move |_| {
                                        value.set(filter);
                                        open.set(false);
                                    },
                                    "{filter.label()}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DeliveryFilterSelect(mut value: Signal<DeliveryFilter>) -> Element {
    let _lang = i18n();
    let mut open = use_signal(|| false);
    let options = [
        DeliveryFilter::All,
        DeliveryFilter::Delivered,
        DeliveryFilter::Queued,
        DeliveryFilter::Issue,
    ];
    let display = value().label();
    let is_open = open();

    rsx! {
        div {
            class: if is_open { "ui-dropdown is-open" } else { "ui-dropdown" },
            button {
                r#type: "button",
                class: if is_open { "ui-dropdown-trigger ui-squircle is-open" } else { "ui-dropdown-trigger ui-squircle" },
                "aria-haspopup": "listbox",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |_| {
                    let next = !open();
                    open.set(next);
                },
                span { class: "ui-dropdown-value", "{display}" }
                span { class: "ui-dropdown-caret", "aria-hidden": "true" }
            }
            if is_open {
                div { class: "ui-dropdown-menu ui-squircle", role: "listbox",
                    for filter in options {
                        {
                            let selected = value() == filter;
                            rsx! {
                                button {
                                    key: "{filter.store_key()}",
                                    r#type: "button",
                                    class: if selected { "ui-dropdown-option is-selected" } else { "ui-dropdown-option" },
                                    role: "option",
                                    "aria-selected": if selected { "true" } else { "false" },
                                    onclick: move |_| {
                                        value.set(filter);
                                        open.set(false);
                                    },
                                    "{filter.label()}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductMediaGallery(mut media: Signal<Vec<String>>) -> Element {
    let _lang = i18n();
    let items = media();
    let count = items.len();

    rsx! {
        div { class: "space-y-2",
            div { class: "flex items-start justify-between gap-3",
                div {
                    p { class: "text-xs font-medium text-text-secondary", { t!("store-media-title") } }
                    p { class: "text-xs text-text-muted", { t!("store-media-hint") } }
                }
                if count > 0 {
                    Button {
                        variant: ButtonVariant::Danger,
                        size: ButtonSize::Sm,
                        onclick: move |_| media.set(Vec::new()),
                        { t!("store-media-clear-all") }
                    }
                }
            }
            div { class: "motion-cascade store-media-grid",
                for (index, src) in items.into_iter().enumerate() {
                    {
                        let remove_index = index;
                        rsx! {
                            div { class: if index == 0 { "store-media-thumb is-cover" } else { "store-media-thumb" },
                                img { src: "{src}", alt: "", class: "store-media-thumb-img" }
                                if index == 0 {
                                    span { class: "store-media-cover-label", { t!("store-media-cover") } }
                                }
                                button {
                                    r#type: "button",
                                    class: "store-media-remove",
                                    title: t!("store-media-remove"),
                                    onclick: move |_| {
                                        media.with_mut(|list| {
                                            if remove_index < list.len() {
                                                list.remove(remove_index);
                                            }
                                        });
                                    },
                                    "×"
                                }
                            }
                        }
                    }
                }
                label { class: "store-media-add",
                    span { class: "pointer-events-none text-center text-xs leading-relaxed text-text-muted",
                        { t!("store-media-upload") }
                    }
                    input {
                        r#type: "file",
                        accept: "image/png,image/jpeg,image/webp,image/gif",
                        multiple: true,
                        class: "absolute inset-0 cursor-pointer opacity-0",
                        onchange: move |evt| {
                            async move {
                                let files: Vec<_> = evt.files().into_iter().collect();
                                for file in files {
                                    let mime = file
                                        .content_type()
                                        .unwrap_or_else(|| String::from("image/png"));
                                    let Ok(bytes) = file.read_bytes().await else {
                                        continue;
                                    };
                                    use base64::Engine as _;
                                    let encoded =
                                        base64::engine::general_purpose::STANDARD.encode(&bytes);
                                    let data_url = format!("data:{mime};base64,{encoded}");
                                    media.with_mut(|list| list.push(data_url));
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn StoreOverview() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let mut hovered_day = use_signal(|| Option::<usize>::None);
    let max_bar = *WEEK_REVENUE.iter().max().unwrap_or(&1);
    let week_total: u32 = WEEK_REVENUE.iter().sum();
    let peak_i = WEEK_REVENUE
        .iter()
        .enumerate()
        .max_by_key(|(_, value)| *value)
        .map(|(i, _)| i)
        .unwrap_or(0);
    let focus_i = hovered_day().unwrap_or(WEEK_REVENUE.len().saturating_sub(1));
    let focus_value = WEEK_REVENUE.get(focus_i).copied().unwrap_or(0);
    let peak_day = t_key(WEEKDAY_FULL_KEYS[peak_i]);
    let focus_label = WEEKDAY_FULL_KEYS
        .get(focus_i)
        .map(|key| t_key(key))
        .unwrap_or_else(|| t_key("dashboard-day-today"));

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("store-overview-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    { t!("store-overview-title") }
                }
            }
            div { class: "flex flex-wrap gap-2",
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, { t!("store-overview-view-site") } }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreProducts {});
                    },
                    IconPlus {}
                    { t!("store-overview-add-product") }
                }
            }
        }

        section { class: "store-hero mb-6",
            div { class: "store-hero-main",
                p { class: "text-sm text-text-muted", { t!("store-overview-revenue-month") } }
                p { class: "mt-2 text-5xl font-semibold tabular-nums tracking-tight text-success sm:text-6xl",
                    "£4,281"
                }
                p { class: "mt-2 text-sm text-text-secondary",
                    { t!("store-overview-week-summary", amount: week_total, day: peak_day) }
                }
            }
            div { class: "store-hero-side",
                div { class: "flex items-start justify-between gap-3",
                    div {
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            { t!("store-overview-this-week") }
                        }
                        p { class: "mt-1 text-sm text-text-secondary",
                            if hovered_day().is_some() {
                                "{focus_label}"
                            } else {
                                { t!("store-overview-hover-day") }
                            }
                        }
                    }
                    div { class: "text-right",
                        p { class: "text-lg font-semibold tabular-nums tracking-tight text-text",
                            "£{focus_value}"
                        }
                        p { class: "text-[11px] text-text-muted", { t!("store-overview-daily-revenue") } }
                    }
                }
                div {
                    class: "motion-cascade motion-cascade-tight store-week-bars mt-4",
                    onmouseleave: move |_| hovered_day.set(None),
                    for (i, value) in WEEK_REVENUE.iter().enumerate() {
                        {
                            let height = ((*value as f32 / max_bar as f32) * 100.0)
                                .clamp(8.0, 100.0)
                                .round() as u32;
                            let today = i + 1 == WEEK_REVENUE.len();
                            let active = hovered_day() == Some(i) || (hovered_day().is_none() && today);
                            let day = t_key(WEEKDAY_KEYS[i]);
                            let full = t_key(WEEKDAY_FULL_KEYS[i]);
                            let amount = *value;
                            rsx! {
                                button {
                                    r#type: "button",
                                    class: if active { "store-week-col is-active" } else { "store-week-col" },
                                    "aria-label": "{full}: £{amount}",
                                    onmouseenter: move |_| hovered_day.set(Some(i)),
                                    onfocus: move |_| hovered_day.set(Some(i)),
                                    span { class: "store-week-tooltip", "{day} · £{amount}" }
                                    span {
                                        class: if today { "store-week-bar is-today" } else { "store-week-bar" },
                                        style: "height: {height}%;",
                                    }
                                    span { class: "store-week-label", "{day}" }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "motion-cascade mb-8 grid gap-3 sm:grid-cols-3",
            button {
                class: "store-pulse-card",
                onclick: move |_| {
                    navigator.push(Route::StoreOrders {});
                },
                p { class: "text-xs text-text-muted", { t!("store-overview-orders-today") } }
                p { class: "mt-2 text-3xl font-semibold tabular-nums", "14" }
                p { class: "mt-1 text-xs text-success", { t!("store-overview-waiting-delivery", count: 3) } }
            }
            button {
                class: "store-pulse-card",
                onclick: move |_| {
                    navigator.push(Route::StoreProducts {});
                },
                p { class: "text-xs text-text-muted", { t!("store-overview-top-seller") } }
                p { class: "mt-2 text-xl font-semibold tracking-tight", "VIP Rank" }
                p { class: "mt-1 text-xs text-text-secondary", "42 sold · £1,260" }
            }
            button {
                class: "store-pulse-card store-pulse-warn",
                onclick: move |_| {
                    navigator.push(Route::StoreSettings {});
                },
                p { class: "text-xs text-text-muted", { t!("store-overview-attention") } }
                p { class: "mt-2 text-xl font-semibold tracking-tight", { t!("store-overview-paypal-sandbox") } }
                p { class: "mt-1 text-xs text-warning", { t!("store-overview-switch-live-keys") } }
            }
        }

        section { class: "mb-3 flex items-baseline justify-between gap-3",
            h2 { class: "text-sm font-semibold text-text", { t!("store-overview-just-checked-out") } }
            button {
                class: "text-xs text-text-muted transition-colors hover:text-text",
                onclick: move |_| {
                    navigator.push(Route::StoreOrders {});
                },
                { t!("store-overview-all-orders") }
            }
        }
        div { class: "motion-cascade motion-cascade-tight store-checkout-feed",
            StoreFeedRow {
                title: "NovaCraft bought VIP Rank",
                meta: "Stripe · Survival delivery",
                amount: "£29.99",
                when: "2m",
                email: "novacraft@players.serverspot.app",
            }
            StoreFeedRow {
                title: "SkyBuilder bought Crate Keys",
                meta: "PayPal · Skyblock",
                amount: "£9.99",
                when: "18m",
                email: "skybuilder@players.serverspot.app",
            }
            StoreFeedRow {
                title: "AetherFox refunded Cosmetics Pack",
                meta: "Stripe · Revoked",
                amount: "£14.99",
                when: "1h",
                email: "aetherfox@players.serverspot.app",
            }
        }
    }
}

#[component]
fn StoreFeedRow(
    #[props(into)] title: String,
    #[props(into)] meta: String,
    #[props(into)] amount: String,
    #[props(into)] when: String,
    #[props(into)] email: String,
) -> Element {
    let _lang = i18n();
    rsx! {
        div { class: "store-checkout-row",
            Avatar { email, size: 36, alt: title.clone() }
            div { class: "min-w-0 flex-1",
                p { class: "truncate text-sm font-medium text-text", "{title}" }
                p { class: "mt-0.5 truncate text-xs text-text-muted", "{meta}" }
            }
            div { class: "shrink-0 text-right",
                p { class: "text-sm font-semibold tabular-nums text-success", "{amount}" }
                p { class: "mt-0.5 text-xs text-text-muted", "{when}" }
            }
        }
    }
}

#[component]
pub fn StoreProducts() -> Element {
    let _lang = i18n();
    let products = use_context::<Signal<Vec<Product>>>();
    let categories = use_context::<Signal<Vec<Category>>>();
    let navigator = use_navigator();
    let query = use_signal(String::new);
    let mut collapsed = use_signal(std::collections::HashSet::<String>::new);

    let groups = use_memo(move || {
        let q = query().to_lowercase();
        let filtered: Vec<Product> = products
            .read()
            .iter()
            .filter(|product| {
                q.is_empty()
                    || product.name.to_lowercase().contains(&q)
                    || product.category.to_lowercase().contains(&q)
            })
            .cloned()
            .collect();

        let mut order: Vec<String> = categories
            .read()
            .iter()
            .map(|category| category.name.clone())
            .collect();
        for product in &filtered {
            if !order.iter().any(|name| name == &product.category) {
                order.push(product.category.clone());
            }
        }

        order
            .into_iter()
            .filter_map(|name| {
                let items: Vec<Product> = filtered
                    .iter()
                    .filter(|product| product.category == name)
                    .cloned()
                    .collect();
                if items.is_empty() {
                    None
                } else {
                    Some((name, items))
                }
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "mb-6 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("store-products-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", { t!("store-products-title") } }
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::StoreProductNew {});
                },
                IconPlus {}
                { t!("store-action-add-product") }
            }
        }

        div { class: "mb-6",
            SearchInput {
                class: "lg:max-w-sm",
                value: query,
                placeholder: t!("store-products-search-placeholder"),
            }
        }

        div { class: "store-product-groups",
            for (category_name, items) in groups().into_iter() {
                {
                    let section_key = category_name.clone();
                    let is_collapsed = collapsed().contains(&category_name);
                    let count = items.len();
                    rsx! {
                        section { key: "{category_name}", class: "store-product-group",
                            button {
                                r#type: "button",
                                class: if is_collapsed { "store-product-group-header is-collapsed" } else { "store-product-group-header" },
                                onclick: move |_| {
                                    collapsed
                                        .with_mut(|set| {
                                            if !set.remove(&section_key) {
                                                set.insert(section_key.clone());
                                            }
                                        });
                                },
                                span { class: "store-product-group-title", "{category_name}" }
                                span { class: "store-product-group-rule", "aria-hidden": "true" }
                                span { class: "store-product-group-meta", "{count}" }
                                span { class: "store-product-group-caret", "aria-hidden": "true" }
                            }
                            if !is_collapsed {
                                div { class: "motion-cascade store-product-grid",
                                    for product in items.into_iter() {
                                        {
                                            let product_id = product.id;
                                            let low = product.stock == "4";
                                            rsx! {
                                                button {
                                                    key: "{product_id}",
                                                    class: "store-product-tile",
                                                    onclick: move |_| {
                                                        navigator
                                                            .push(Route::StoreProductEdit {
                                                                id: product_id,
                                                            });
                                                    },
                                                    div { class: "store-product-tile-top",
                                                        if !product.visible {
                                                            span { class: "text-xs text-text-muted", { t!("store-product-hidden") } }
                                                        } else if low {
                                                            span { class: "text-xs text-warning", { t!("store-product-low-stock") } }
                                                        } else {
                                                            span { class: "text-xs text-text-muted", { t!("store-product-sold-count", count: product.sold) } }
                                                        }
                                                    }
                                                    p { class: "mt-3 text-lg font-semibold tracking-tight text-text", "{product.name}" }
                                                    div { class: "mt-auto flex items-end justify-between gap-3 pt-6",
                                                        p { class: "text-2xl font-semibold tabular-nums tracking-tight", "{product.price}" }
                                                        p { class: "text-xs text-text-muted", "{product.stock}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn StoreProductNew() -> Element {
    let _lang = i18n();
    rsx! {
        ProductEditor { product_id: None }
    }
}

#[component]
pub fn StoreProductEdit(id: u64) -> Element {
    let _lang = i18n();
    rsx! {
        ProductEditor { product_id: Some(id) }
    }
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !slug.is_empty() && !dash {
            slug.push('-');
            dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

#[component]
fn ProductEditor(product_id: Option<u64>) -> Element {
    let _lang = i18n();
    let mut products = use_context::<Signal<Vec<Product>>>();
    let navigator = use_navigator();
    let is_new = product_id.is_none();

    let (seed, missing) = use_hook(|| {
        let existing = product_id.and_then(|id| {
            products
                .peek()
                .iter()
                .find(|product| product.id == id)
                .cloned()
        });
        let missing = !is_new && existing.is_none();
        let seed = existing.unwrap_or_else(Product::blank);
        (seed, missing)
    });

    let mut name = use_signal(|| seed.name.clone());
    let mut slug = use_signal(|| seed.slug.clone());
    let mut description = use_signal(|| seed.description.clone());
    let mut category = use_signal(|| seed.category.clone());
    let mut price = use_signal(|| seed.price.clone());
    let mut sale_price = use_signal(|| seed.sale_price.clone());
    let mut stock = use_signal(|| seed.stock.clone());
    let mut visible = use_signal(|| seed.visible);
    let mut featured = use_signal(|| seed.featured);
    let mut giftable = use_signal(|| seed.giftable);
    let mut require_online = use_signal(|| seed.require_online);
    let mut servers = use_signal(|| seed.servers.clone());
    let mut commands = use_signal(|| seed.commands.clone());
    let mut revoke_commands = use_signal(|| seed.revoke_commands.clone());
    let mut expiry_days = use_signal(|| seed.expiry_days.clone());
    let mut media = use_signal(|| seed.media.clone());
    let sold = seed.sold;
    let mut slug_touched = use_signal(|| !is_new);

    use_effect(move || {
        let next = name();
        if !*slug_touched.peek() {
            slug.set(slugify(&next));
        }
    });

    if missing {
        return rsx! {
            div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreProducts {});
                    },
                    { t!("store-back-products") }
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", { t!("store-product-not-found") } }
            p { class: "mt-2 text-sm text-text-muted", { t!("store-product-not-found-hint") } }
        };
    }

    let categories = use_context::<Signal<Vec<Category>>>();
    let category_options: Vec<String> = categories()
        .into_iter()
        .map(|c| c.name)
        .chain(std::iter::once(String::from("Uncategorised")))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    let save = move |_| {
        let trimmed_name = name().trim().to_string();
        if trimmed_name.is_empty() {
            return;
        }
        let mut resolved_slug = slug().trim().to_string();
        if resolved_slug.is_empty() {
            resolved_slug = slugify(&trimmed_name);
        }
        let draft = Product {
            id: product_id.unwrap_or(0),
            name: trimmed_name,
            slug: resolved_slug,
            description: description().trim().to_string(),
            category: {
                let value = category().trim().to_string();
                if value.is_empty() {
                    String::from("Uncategorised")
                } else {
                    value
                }
            },
            price: price().trim().to_string(),
            sale_price: sale_price().trim().to_string(),
            stock: stock().trim().to_string(),
            visible: visible(),
            featured: featured(),
            giftable: giftable(),
            require_online: require_online(),
            servers: servers(),
            commands: commands().trim().to_string(),
            revoke_commands: revoke_commands().trim().to_string(),
            expiry_days: expiry_days().trim().to_string(),
            sold,
            media: media(),
        };

        products.with_mut(|list| {
            if let Some(id) = product_id {
                if let Some(item) = list.iter_mut().find(|p| p.id == id) {
                    let keep_sold = item.sold;
                    *item = draft;
                    item.id = id;
                    item.sold = keep_sold;
                }
            } else {
                let next_id = list.iter().map(|p| p.id).max().unwrap_or(0) + 1;
                let mut created = draft;
                created.id = next_id;
                created.sold = 0;
                list.push(created);
            }
        });
        navigator.push(Route::StoreProducts {});
    };

    rsx! {
        div { class: "motion-cascade store-editor",
            div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreProducts {});
                    },
                    { t!("store-back-products") }
                }
                div { class: "flex flex-wrap items-center gap-2",
                    if let Some(id) = product_id {
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                products.with_mut(|list| list.retain(|p| p.id != id));
                                navigator.push(Route::StoreProducts {});
                            },
                            { t!("common-delete") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::StoreProducts {});
                        },
                        { t!("common-cancel") }
                    }
                    Button { size: ButtonSize::Sm, onclick: save,
                        if is_new {
                            { t!("store-action-create-product") }
                        } else {
                            { t!("common-save-changes") }
                        }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", { t!("store-products-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight",
                    if is_new {
                        { t!("store-product-new-title") }
                    } else {
                        { t!("store-product-edit-title") }
                    }
                }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted",
                    { t!("store-product-editor-description") }
                }
            }

            div { class: "store-editor-layout",
                div { class: "store-editor-main space-y-8",
                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-basics") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-basics-lede") }
                        }
                        div { class: "mt-4 space-y-4",
                            StoreFormField { label: t!("store-field-name"),
                                SignalInput { value: name, placeholder: "VIP Rank" }
                            }
                            div { class: "motion-cascade grid gap-4 sm:grid-cols-2",
                                StoreFormField { label: t!("store-field-slug"),
                                    input {
                                        r#type: "text",
                                        class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                                        value: "{slug}",
                                        placeholder: "vip-rank",
                                        oninput: move |evt: FormEvent| {
                                            slug_touched.set(true);
                                            slug.set(evt.value());
                                        },
                                    }
                                }
                                StoreFormField { label: t!("store-field-category"),
                                    SignalSelect {
                                        value: category,
                                        options: category_options
                                            .iter()
                                            .cloned()
                                            .map(|name| SelectOption::new(name.clone(), name))
                                            .collect(),
                                    }
                                }
                            }
                            StoreFormField { label: t!("store-field-description"),
                                SignalTextarea {
                                    value: description,
                                    placeholder: t!("store-placeholder-description"),
                                }
                            }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-media") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-media-lede") }
                        }
                        div { class: "mt-4",
                            ProductMediaGallery { media }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-pricing") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-pricing-lede") }
                        }
                        div { class: "mt-4 grid gap-4 sm:grid-cols-3",
                            StoreFormField { label: t!("store-field-price"),
                                SignalInput { value: price, placeholder: "£29.99" }
                            }
                            StoreFormField { label: t!("store-field-sale-price"),
                                SignalInput {
                                    value: sale_price,
                                    placeholder: t!("store-placeholder-optional"),
                                }
                            }
                            StoreFormField { label: t!("store-field-stock"),
                                SignalInput { value: stock, placeholder: "Unlimited" }
                            }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-delivery") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-delivery-lede") }
                        }
                        div { class: "mt-4 space-y-4",
                            StoreFormField { label: t!("store-field-target-servers"),
                                SignalMultiSelect {
                                    value: servers,
                                    placeholder: t!("store-placeholder-select-servers"),
                                    options: ["Lobby", "Survival", "Skyblock", "Creative"]
                                        .into_iter()
                                        .map(|name| SelectOption::new(name, name))
                                        .collect(),
                                }
                            }
                            StoreFormField { label: t!("store-field-delivery-commands"),
                                SignalTextarea {
                                    value: commands,
                                    placeholder: "lp user {{player}} parent add vip",
                                }
                            }
                            StoreFormField { label: t!("store-field-revoke-commands"),
                                SignalTextarea {
                                    value: revoke_commands,
                                    placeholder: "lp user {{player}} parent remove vip",
                                }
                            }
                            StoreFormField { label: t!("store-field-expires-days"),
                                SignalInput {
                                    value: expiry_days,
                                    placeholder: t!("store-placeholder-permanent"),
                                }
                            }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-visibility") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-visibility-lede") }
                        }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            ProductToggleRow {
                                title: t!("store-toggle-show-in-store"),
                                description: t!("store-toggle-show-in-store-hint"),
                                enabled: visible,
                            }
                            ProductToggleRow {
                                title: t!("store-toggle-featured"),
                                description: t!("store-toggle-featured-hint"),
                                enabled: featured,
                            }
                            ProductToggleRow {
                                title: t!("store-toggle-allow-gifts"),
                                description: t!("store-toggle-allow-gifts-hint"),
                                enabled: giftable,
                            }
                            ProductToggleRow {
                                title: t!("store-toggle-require-online"),
                                description: t!("store-toggle-require-online-hint"),
                                enabled: require_online,
                            }
                        }
                    }
                }

                aside { class: "store-editor-aside",
                    div { class: "store-editor-summary",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            { t!("store-preview") }
                        }
                        p { class: "mt-3 text-xl font-semibold tracking-tight text-text",
                            if name().trim().is_empty() {
                                { t!("store-preview-untitled-product") }
                            } else {
                                "{name}"
                            }
                        }
                        p { class: "mt-1 text-xs text-text-muted", "/store/{slug}" }
                        p { class: "mt-4 text-xs text-text-secondary", "{category}" }
                        p { class: "mt-5 text-2xl font-semibold tabular-nums tracking-tight",
                            "{price}"
                        }
                        if !sale_price().trim().is_empty() {
                            p { class: "mt-1 text-sm text-accent", { t!("store-preview-sale", price: sale_price()) } }
                        }
                        p { class: "mt-4 text-xs text-text-muted", { t!("store-preview-stock", stock: stock()) } }
                        if !is_new {
                            p { class: "mt-1 text-xs text-text-muted", { t!("store-product-sold-count", count: sold) } }
                        }
                        ul { class: "mt-5 space-y-1.5 text-xs text-text-secondary",
                            li {
                                if visible() {
                                    { t!("store-preview-visible") }
                                } else {
                                    { t!("store-preview-hidden") }
                                }
                            }
                            li {
                                if featured() {
                                    { t!("store-preview-featured") }
                                } else {
                                    { t!("store-preview-standard") }
                                }
                            }
                            li {
                                if giftable() {
                                    { t!("store-preview-gifting-on") }
                                } else {
                                    { t!("store-preview-gifting-off") }
                                }
                            }
                            li {
                                if require_online() {
                                    { t!("store-preview-requires-online") }
                                } else {
                                    { t!("store-preview-offline-ok") }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductToggleRow(
    #[props(into)] title: String,
    #[props(into)] description: String,
    mut enabled: Signal<bool>,
) -> Element {
    let _lang = i18n();
    rsx! {
        div { class: "flex items-start justify-between gap-4 py-4",
            div { class: "min-w-0",
                p { class: "text-sm font-medium text-text", "{title}" }
                p { class: "mt-1 text-sm text-text-muted", "{description}" }
            }
            Button {
                class: "self-start",
                variant: if enabled() { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                size: ButtonSize::Sm,
                onclick: move |_| {
                    let next = !*enabled.peek();
                    enabled.set(next);
                },
                if enabled() {
                    { t!("common-on") }
                } else {
                    { t!("common-off") }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub note: String,
    pub accent: String,
    pub product_count: u32,
    pub visible: bool,
    pub cumulative: bool,
    pub parent: String,
}

impl Category {
    fn blank() -> Self {
        Self {
            id: 0,
            name: String::new(),
            note: String::new(),
            accent: String::from(STORE_ACCENT),
            product_count: 0,
            visible: true,
            cumulative: false,
            parent: String::new(),
        }
    }
}

pub(crate) fn placeholder_categories() -> Vec<Category> {
    vec![
        Category {
            id: 1,
            name: String::from("Ranks"),
            note: String::from("Cumulative upgrades on"),
            accent: String::from("#5b9dff"),
            product_count: 8,
            visible: true,
            cumulative: true,
            parent: String::new(),
        },
        Category {
            id: 2,
            name: String::from("Crates"),
            note: String::from("Keys & openers"),
            accent: String::from("#f0a35e"),
            product_count: 6,
            visible: true,
            cumulative: false,
            parent: String::new(),
        },
        Category {
            id: 3,
            name: String::from("Cosmetics"),
            note: String::from("Trails and pets"),
            accent: String::from("#f071a5"),
            product_count: 5,
            visible: true,
            cumulative: false,
            parent: String::new(),
        },
        Category {
            id: 4,
            name: String::from("Perks"),
            note: String::from("Homes, fly, kits"),
            accent: String::from("#5eead4"),
            product_count: 4,
            visible: true,
            cumulative: false,
            parent: String::new(),
        },
        Category {
            id: 5,
            name: String::from("Bundles"),
            note: String::from("Seasonal packs"),
            accent: String::from(STORE_ACCENT),
            product_count: 3,
            visible: true,
            cumulative: false,
            parent: String::new(),
        },
        Category {
            id: 6,
            name: String::from("Legacy ranks"),
            note: String::from("Archived · child of Ranks"),
            accent: String::from("#71717a"),
            product_count: 2,
            visible: false,
            cumulative: false,
            parent: String::from("Ranks"),
        },
    ]
}

#[component]
pub fn StoreCategories() -> Element {
    let _lang = i18n();
    let categories = use_context::<Signal<Vec<Category>>>();
    let navigator = use_navigator();
    let list = categories();

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("store-categories-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", { t!("store-categories-title") } }
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::StoreCategoryNew {});
                },
                IconPlus {}
                { t!("store-action-add-category") }
            }
        }

        p { class: "mb-6 max-w-xl text-sm text-text-secondary",
            { t!("store-categories-description") }
        }

        div { class: "motion-cascade store-category-lanes",
            for category in list.iter().cloned() {
                {
                    let category_id = category.id;
                    let count = category.product_count.to_string();
                    rsx! {
                        button {
                            class: if category.visible { "store-category-lane store-category-lane-btn" } else { "store-category-lane store-category-lane-btn is-hidden" },
                            style: "--lane-accent: {category.accent};",
                            onclick: move |_| {
                                navigator
                                    .push(Route::StoreCategoryEdit {
                                        id: category_id,
                                    });
                            },
                            div { class: "store-category-lane-bar" }
                            div { class: "min-w-0 flex-1 text-left",
                                p { class: "text-base font-semibold tracking-tight text-text", "{category.name}" }
                                p { class: "mt-1 text-xs text-text-muted", "{category.note}" }
                                if category.cumulative || !category.parent.is_empty() {
                                    p { class: "mt-1.5 text-[11px] text-text-secondary",
                                        if category.cumulative && !category.parent.is_empty() {
                                            { t!("store-category-cumulative-child", parent: category.parent.clone()) }
                                        } else if category.cumulative {
                                            { t!("store-category-cumulative") }
                                        } else {
                                            { t!("store-category-child-of", parent: category.parent.clone()) }
                                        }
                                    }
                                }
                            }
                            div { class: "shrink-0 text-right",
                                p { class: "text-3xl font-semibold tabular-nums tracking-tight", "{count}" }
                                p { class: "mt-0.5 text-[11px] text-text-muted",
                                    if category.visible {
                                        { t!("store-category-products-edit") }
                                    } else {
                                        { t!("store-category-hidden-edit") }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn StoreCategoryNew() -> Element {
    let _lang = i18n();
    rsx! {
        CategoryEditor { category_id: None }
    }
}

#[component]
pub fn StoreCategoryEdit(id: u64) -> Element {
    let _lang = i18n();
    rsx! {
        CategoryEditor { category_id: Some(id) }
    }
}

#[component]
fn CategoryEditor(category_id: Option<u64>) -> Element {
    let _lang = i18n();
    let mut categories = use_context::<Signal<Vec<Category>>>();
    let mut products = use_context::<Signal<Vec<Product>>>();
    let navigator = use_navigator();
    let is_new = category_id.is_none();

    let existing = category_id.and_then(|id| {
        categories
            .read()
            .iter()
            .find(|category| category.id == id)
            .cloned()
    });
    let missing = !is_new && existing.is_none();
    let seed = existing.clone().unwrap_or_else(Category::blank);
    let original_name = seed.name.clone();
    let product_count = seed.product_count;

    let mut name = use_signal(|| seed.name.clone());
    let mut note = use_signal(|| seed.note.clone());
    let mut accent = use_signal(|| seed.accent.clone());
    let mut parent = use_signal(|| seed.parent.clone());
    let mut visible = use_signal(|| seed.visible);
    let mut cumulative = use_signal(|| seed.cumulative);

    let parent_options: Vec<SelectOption> = std::iter::once(SelectOption::new("", "None"))
        .chain(
            categories()
                .into_iter()
                .filter(|c| Some(c.id) != category_id)
                .map(|c| SelectOption::new(c.name.clone(), c.name)),
        )
        .collect();

    if missing {
        return rsx! {
            div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreCategories {});
                    },
                    { t!("store-back-categories") }
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", { t!("store-category-not-found") } }
            p { class: "mt-2 text-sm text-text-muted", { t!("store-category-not-found-hint") } }
        };
    }

    let save = move |_| {
        let trimmed = name().trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        let resolved_accent = {
            let value = accent().trim().to_string();
            if value.is_empty() {
                String::from(STORE_ACCENT)
            } else {
                value
            }
        };
        let next_parent = parent().trim().to_string();
        let next_note = note().trim().to_string();
        let next_visible = visible();
        let next_cumulative = cumulative();

        categories.with_mut(|list| {
            if let Some(id) = category_id {
                let old_name = list
                    .iter()
                    .find(|c| c.id == id)
                    .map(|c| c.name.clone())
                    .unwrap_or_default();
                if let Some(item) = list.iter_mut().find(|c| c.id == id) {
                    item.name = trimmed.clone();
                    item.note = next_note.clone();
                    item.accent = resolved_accent.clone();
                    item.parent = next_parent.clone();
                    item.visible = next_visible;
                    item.cumulative = next_cumulative;
                }
                if old_name != trimmed {
                    for category in list.iter_mut() {
                        if category.parent == old_name {
                            category.parent = trimmed.clone();
                        }
                    }
                }
            } else {
                let next_id = list.iter().map(|c| c.id).max().unwrap_or(0) + 1;
                list.push(Category {
                    id: next_id,
                    name: trimmed.clone(),
                    note: next_note,
                    accent: resolved_accent,
                    product_count: 0,
                    visible: next_visible,
                    cumulative: next_cumulative,
                    parent: next_parent,
                });
            }
        });

        if let Some(_) = category_id {
            if !original_name.is_empty() && original_name != trimmed {
                products.with_mut(|list| {
                    for product in list.iter_mut() {
                        if product.category == original_name {
                            product.category = trimmed.clone();
                        }
                    }
                });
            }
        }

        navigator.push(Route::StoreCategories {});
    };

    rsx! {
        div { class: "motion-cascade store-editor",
            div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreCategories {});
                    },
                    { t!("store-back-categories") }
                }
                div { class: "flex flex-wrap items-center gap-2",
                    if let Some(id) = category_id {
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                let removed_name = categories()
                                    .into_iter()
                                    .find(|c| c.id == id)
                                    .map(|c| c.name)
                                    .unwrap_or_default();
                                categories
                                    .with_mut(|list| {
                                        list.retain(|c| c.id != id);
                                        for category in list.iter_mut() {
                                            if category.parent == removed_name {
                                                category.parent.clear();
                                            }
                                        }
                                    });
                                navigator.push(Route::StoreCategories {});
                            },
                            { t!("common-delete") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::StoreCategories {});
                        },
                        { t!("common-cancel") }
                    }
                    Button { size: ButtonSize::Sm, onclick: save,
                        if is_new {
                            { t!("store-action-create-category") }
                        } else {
                            { t!("common-save-changes") }
                        }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", { t!("store-categories-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight",
                    if is_new {
                        { t!("store-category-new-title") }
                    } else {
                        { t!("store-category-edit-title") }
                    }
                }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted",
                    { t!("store-category-editor-description") }
                }
            }

            div { class: "store-editor-layout",
                div { class: "store-editor-main space-y-8",
                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-basics") } }
                        p { class: "store-editor-lede",
                            { t!("store-category-basics-lede") }
                        }
                        div { class: "mt-4 space-y-4",
                            StoreFormField { label: t!("store-field-name"),
                                SignalInput { value: name, placeholder: "Ranks" }
                            }
                            StoreFormField { label: t!("store-field-description"),
                                SignalInput {
                                    value: note,
                                    placeholder: "Keys & openers",
                                }
                            }
                            StoreFormField { label: t!("store-field-parent-category"),
                                SignalSelect {
                                    value: parent,
                                    options: parent_options,
                                    placeholder: t!("store-placeholder-none"),
                                }
                            }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-appearance") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-appearance-lede") }
                        }
                        div { class: "mt-4",
                            StoreFormField { label: t!("store-field-accent-colour"),
                                ColorPicker { value: accent }
                            }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-visibility-pricing") } }
                        p { class: "store-editor-lede", { t!("store-section-visibility-pricing-lede") } }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            ProductToggleRow {
                                title: t!("store-toggle-show-in-store"),
                                description: t!("store-toggle-show-category-hint"),
                                enabled: visible,
                            }
                            ProductToggleRow {
                                title: t!("store-toggle-cumulative"),
                                description: t!("store-toggle-cumulative-hint"),
                                enabled: cumulative,
                            }
                        }
                    }
                }

                aside { class: "store-editor-aside",
                    div { class: "store-editor-summary",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            { t!("store-preview") }
                        }
                        div {
                            class: "mt-4 store-category-lane",
                            style: "--lane-accent: {accent};",
                            div { class: "store-category-lane-bar" }
                            div { class: "min-w-0 flex-1",
                                p { class: "text-base font-semibold tracking-tight text-text",
                                    if name().trim().is_empty() {
                                        { t!("store-preview-untitled-category") }
                                    } else {
                                        "{name}"
                                    }
                                }
                                p { class: "mt-1 text-xs text-text-muted",
                                    if note().trim().is_empty() {
                                        { t!("store-preview-no-description") }
                                    } else {
                                        "{note}"
                                    }
                                }
                            }
                        }
                        ul { class: "mt-5 space-y-1.5 text-xs text-text-secondary",
                            li {
                                if parent().trim().is_empty() {
                                    { t!("store-preview-top-level") }
                                } else {
                                    { t!("store-preview-child-of", parent: parent()) }
                                }
                            }
                            li {
                                if visible() {
                                    { t!("store-preview-visible-store") }
                                } else {
                                    { t!("store-preview-hidden-store") }
                                }
                            }
                            li {
                                if cumulative() {
                                    { t!("store-preview-cumulative-on") }
                                } else {
                                    { t!("store-preview-standard-pricing") }
                                }
                            }
                            if !is_new {
                                li { { t!("store-preview-products-count", count: product_count) } }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Coupon {
    pub id: u64,
    pub code: String,
    pub offer: String,
    pub discount: String,
    pub min_order: String,
    pub accent: String,
    pub active: bool,
    pub ends: String,
    pub uses: u32,
    pub note: String,
}

impl Coupon {
    fn blank() -> Self {
        Self {
            id: 0,
            code: String::new(),
            offer: String::new(),
            discount: String::from("10"),
            min_order: String::from("0.00"),
            accent: String::from(STORE_ACCENT),
            active: true,
            ends: String::new(),
            uses: 0,
            note: String::new(),
        }
    }

    fn meta_line(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        let min = self.min_order.trim();
        if !min.is_empty() && min != "0" && min != "0.00" {
            parts.push(t!("store-coupon-meta-min", amount: min.to_string()));
        }
        if !self.ends.trim().is_empty() {
            parts.push(t!(
                "store-coupon-meta-ends",
                date: format_display_date(self.ends.trim())
            ));
        }
        if !self.note.trim().is_empty() {
            parts.push(self.note.clone());
        }
        if self.uses > 0 {
            parts.push(t!("store-coupon-meta-uses", count: self.uses));
        }
        if !self.active {
            parts.push(t_key("store-coupon-meta-paused"));
        }
        if parts.is_empty() {
            t_key("store-coupon-meta-no-limits")
        } else {
            parts.join(" · ")
        }
    }
}

pub(crate) fn placeholder_coupons() -> Vec<Coupon> {
    vec![
        Coupon {
            id: 1,
            code: String::from("SUMMER20"),
            offer: String::from("20% off lifetime ranks"),
            discount: String::from("20"),
            min_order: String::from("10.00"),
            accent: String::from(STORE_ACCENT),
            active: true,
            ends: String::new(),
            uses: 8,
            note: String::from("8 uses this week"),
        },
        Coupon {
            id: 2,
            code: String::from("WEEKENDKEYS"),
            offer: String::from("Buy 2 get 1 free on crates"),
            discount: String::from("0"),
            min_order: String::from("0.00"),
            accent: String::from("#f0a35e"),
            active: true,
            ends: String::from("2026-08-02"),
            uses: 0,
            note: String::from("crates only"),
        },
        Coupon {
            id: 3,
            code: String::from("NOVA"),
            offer: String::from("Creator · 10% off · 8% share"),
            discount: String::from("10"),
            min_order: String::from("0.00"),
            accent: String::from("#5b9dff"),
            active: true,
            ends: String::new(),
            uses: 12,
            note: String::from("attributed"),
        },
        Coupon {
            id: 4,
            code: String::from("LAUNCH15"),
            offer: String::from("15% storewide"),
            discount: String::from("15"),
            min_order: String::from("0.00"),
            accent: String::from("#71717a"),
            active: false,
            ends: String::from("2025-06-15"),
            uses: 64,
            note: String::new(),
        },
        Coupon {
            id: 5,
            code: String::from("BUILDER10"),
            offer: String::from("10% cosmetics"),
            discount: String::from("10"),
            min_order: String::from("0.00"),
            accent: String::from("#71717a"),
            active: false,
            ends: String::new(),
            uses: 0,
            note: String::from("Paused by staff"),
        },
    ]
}

#[component]
pub fn StoreCoupons() -> Element {
    let _lang = i18n();
    let coupons = use_context::<Signal<Vec<Coupon>>>();
    let navigator = use_navigator();
    let list_len = use_memo(move || coupons.read().len());
    let active = use_memo(move || {
        coupons
            .read()
            .iter()
            .filter(|c| c.active)
            .cloned()
            .collect::<Vec<_>>()
    });
    let inactive = use_memo(move || {
        coupons
            .read()
            .iter()
            .filter(|c| !c.active)
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("store-coupons-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", { t!("store-coupons-title") } }
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::StoreCouponNew {});
                },
                IconPlus {}
                { t!("store-action-add-coupon") }
            }
        }

        if !active().is_empty() {
            section { class: "mb-8",
                p { class: "mb-3 text-xs font-medium uppercase tracking-wide text-text-muted",
                    { t!("store-coupons-active") }
                }
                div { class: "motion-cascade store-coupon-rack",
                    for coupon in active() {
                        StoreCouponTicket { key: "{coupon.id}", coupon }
                    }
                }
            }
        }

        if !inactive().is_empty() {
            section {
                p { class: "mb-3 text-xs font-medium uppercase tracking-wide text-text-muted",
                    { t!("store-coupons-inactive") }
                }
                div { class: "motion-cascade store-coupon-rack store-coupon-rack-muted",
                    for coupon in inactive() {
                        StoreCouponTicket { key: "{coupon.id}", coupon }
                    }
                }
            }
        }

        if list_len() == 0 {
            p { class: "text-sm text-text-muted", { t!("store-coupons-empty") } }
        }
    }
}

#[component]
pub fn StoreCouponNew() -> Element {
    let _lang = i18n();
    rsx! {
        CouponEditor { coupon_id: None }
    }
}

#[component]
pub fn StoreCouponEdit(id: u64) -> Element {
    let _lang = i18n();
    rsx! {
        CouponEditor { coupon_id: Some(id) }
    }
}

#[component]
fn CouponEditor(coupon_id: Option<u64>) -> Element {
    let _lang = i18n();
    let mut coupons = use_context::<Signal<Vec<Coupon>>>();
    let navigator = use_navigator();
    let is_new = coupon_id.is_none();

    let existing = coupon_id.and_then(|id| {
        coupons
            .read()
            .iter()
            .find(|coupon| coupon.id == id)
            .cloned()
    });
    let missing = !is_new && existing.is_none();
    let seed = existing.clone().unwrap_or_else(Coupon::blank);
    let uses = seed.uses;

    let mut code = use_signal(|| seed.code.clone());
    let mut offer = use_signal(|| seed.offer.clone());
    let mut discount = use_signal(|| seed.discount.clone());
    let mut min_order = use_signal(|| seed.min_order.clone());
    let mut accent = use_signal(|| seed.accent.clone());
    let mut active = use_signal(|| seed.active);
    let mut ends = use_signal(|| seed.ends.clone());
    let mut note = use_signal(|| seed.note.clone());

    if missing {
        return rsx! {
            div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreCoupons {});
                    },
                    { t!("store-back-coupons") }
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", { t!("store-coupon-not-found") } }
            p { class: "mt-2 text-sm text-text-muted", { t!("store-coupon-not-found-hint") } }
        };
    }

    let save = move |_| {
        let trimmed_code = code().trim().to_uppercase();
        if trimmed_code.is_empty() {
            return;
        }
        let resolved_accent = {
            let value = accent().trim().to_string();
            if value.is_empty() {
                String::from(STORE_ACCENT)
            } else {
                value
            }
        };
        let draft = Coupon {
            id: coupon_id.unwrap_or(0),
            code: trimmed_code,
            offer: offer().trim().to_string(),
            discount: discount().trim().to_string(),
            min_order: min_order().trim().to_string(),
            accent: resolved_accent,
            active: active(),
            ends: ends().trim().to_string(),
            uses,
            note: note().trim().to_string(),
        };

        coupons.with_mut(|list| {
            if let Some(id) = coupon_id {
                if let Some(item) = list.iter_mut().find(|c| c.id == id) {
                    let keep_uses = item.uses;
                    *item = draft;
                    item.id = id;
                    item.uses = keep_uses;
                }
            } else {
                let next_id = list.iter().map(|c| c.id).max().unwrap_or(0) + 1;
                let mut created = draft;
                created.id = next_id;
                created.uses = 0;
                list.push(created);
            }
        });
        navigator.push(Route::StoreCoupons {});
    };

    let preview = Coupon {
        id: 0,
        code: {
            let value = code().trim().to_uppercase();
            if value.is_empty() {
                String::from("CODE")
            } else {
                value
            }
        },
        offer: {
            let value = offer().trim().to_string();
            if value.is_empty() {
                t_key("store-coupon-preview-describe-offer")
            } else {
                value
            }
        },
        discount: discount(),
        min_order: min_order(),
        accent: {
            let value = accent().trim().to_string();
            if value.is_empty() {
                String::from(STORE_ACCENT)
            } else {
                value
            }
        },
        active: active(),
        ends: ends(),
        uses,
        note: note(),
    };

    rsx! {
        div { class: "motion-cascade store-editor",
            div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreCoupons {});
                    },
                    { t!("store-back-coupons") }
                }
                div { class: "flex flex-wrap items-center gap-2",
                    if let Some(id) = coupon_id {
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                coupons.with_mut(|list| list.retain(|c| c.id != id));
                                navigator.push(Route::StoreCoupons {});
                            },
                            { t!("common-delete") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::StoreCoupons {});
                        },
                        { t!("common-cancel") }
                    }
                    Button { size: ButtonSize::Sm, onclick: save,
                        if is_new {
                            { t!("store-action-create-coupon") }
                        } else {
                            { t!("common-save-changes") }
                        }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", { t!("store-coupons-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight",
                    if is_new {
                        { t!("store-coupon-new-title") }
                    } else {
                        { t!("store-coupon-edit-title") }
                    }
                }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted",
                    { t!("store-coupon-editor-description") }
                }
            }

            div { class: "store-editor-layout",
                div { class: "store-editor-main space-y-8",
                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-basics") } }
                        p { class: "store-editor-lede",
                            { t!("store-coupon-basics-lede") }
                        }
                        div { class: "mt-4 space-y-4",
                            StoreFormField { label: t!("store-field-coupon-code"),
                                SignalInput { value: code, placeholder: "SUMMER20" }
                            }
                            StoreFormField { label: t!("store-field-offer-summary"),
                                SignalInput {
                                    value: offer,
                                    placeholder: "20% off lifetime ranks",
                                }
                            }
                            div { class: "motion-cascade grid gap-4 sm:grid-cols-2",
                                StoreFormField { label: t!("store-field-discount-percent"),
                                    SignalInput { value: discount, placeholder: "20" }
                                }
                                StoreFormField { label: t!("store-field-min-order"),
                                    SignalInput {
                                        value: min_order,
                                        placeholder: "10.00",
                                    }
                                }
                            }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-limits-notes") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-limits-notes-lede") }
                        }
                        div { class: "mt-4 space-y-4",
                            StoreFormField { label: t!("store-field-ends"),
                                SignalDatePicker {
                                    value: ends,
                                    placeholder: t!("store-placeholder-no-end-date"),
                                    allow_clear: true,
                                }
                            }
                            StoreFormField { label: t!("store-field-note"),
                                SignalInput {
                                    value: note,
                                    placeholder: "crates only · attributed",
                                }
                            }
                        }
                    }

                    section { class: "store-editor-section",
                        h2 { class: "store-editor-heading", { t!("store-section-appearance-status") } }
                        p { class: "store-editor-lede",
                            { t!("store-section-appearance-status-lede") }
                        }
                        div { class: "mt-4 space-y-4",
                            StoreFormField { label: t!("store-field-accent-colour"),
                                ColorPicker { value: accent }
                            }
                            div { class: "divide-y divide-border-subtle border-t border-border-subtle",
                                ProductToggleRow {
                                    title: t!("store-toggle-active"),
                                    description: t!("store-toggle-active-hint"),
                                    enabled: active,
                                }
                            }
                        }
                    }
                }

                aside { class: "store-editor-aside",
                    div { class: "store-editor-summary",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            { t!("store-preview") }
                        }
                        div { class: "mt-4",
                            StoreCouponTicket { coupon: preview }
                        }
                        ul { class: "mt-5 space-y-1.5 text-xs text-text-secondary",
                            li {
                                if discount().trim().is_empty() {
                                    { t!("store-preview-no-percent") }
                                } else {
                                    { t!("store-preview-discount-percent", percent: discount()) }
                                }
                            }
                            li {
                                if active() {
                                    { t!("store-preview-redeemable") }
                                } else {
                                    { t!("store-preview-paused-expired") }
                                }
                            }
                            if !is_new {
                                li { { t!("store-coupon-uses-count", count: uses) } }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StoreCouponTicket(coupon: Coupon) -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let coupon_id = coupon.id;
    let meta = coupon.meta_line();
    let active = coupon.active;

    rsx! {
        button {
            r#type: "button",
            class: if active { "store-coupon-ticket store-coupon-ticket-btn" } else { "store-coupon-ticket store-coupon-ticket-btn is-muted" },
            style: "--ticket-accent: {coupon.accent};",
            onclick: move |_| {
                if coupon_id > 0 {
                    navigator
                        .push(Route::StoreCouponEdit {
                            id: coupon_id,
                        });
                }
            },
            div { class: "store-coupon-ticket-stub",
                p { class: "store-coupon-code", "{coupon.code}" }
            }
            div { class: "store-coupon-ticket-body",
                p { class: "text-sm font-medium text-text", "{coupon.offer}" }
                p { class: "mt-1 text-xs text-text-muted", "{meta}" }
            }
        }
    }
}

#[component]
pub fn StoreOrders() -> Element {
    let _lang = i18n();
    let orders = use_signal(placeholder_orders);
    let mut query = use_signal(String::new);
    let mut status = use_signal(|| OrderStatus::All);
    let mut payment = use_signal(|| PaymentFilter::All);
    let mut delivery = use_signal(|| DeliveryFilter::All);
    let mut server = use_signal(|| ALL_SERVERS_KEY.to_string());
    let mut selected = use_signal(|| Option::<u64>::None);
    let mut visible = use_signal(|| ORDER_PAGE_SIZE);

    use_effect(move || {
        let _ = (query(), status(), payment(), delivery(), server());
        visible.set(ORDER_PAGE_SIZE);
    });

    let server_options = use_memo(move || {
        std::iter::once(ALL_SERVERS_KEY.to_string())
            .chain(
                orders
                    .read()
                    .iter()
                    .map(|order| order.server.clone())
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter(),
            )
            .collect::<Vec<_>>()
    });

    let paid_count = use_memo(move || {
        orders
            .read()
            .iter()
            .filter(|order| order.status == OrderStatus::Paid)
            .count()
    });
    let pending_count = use_memo(move || {
        orders
            .read()
            .iter()
            .filter(|order| order.status == OrderStatus::Pending)
            .count()
    });
    let refunded_count = use_memo(move || {
        orders
            .read()
            .iter()
            .filter(|order| order.status == OrderStatus::Refunded)
            .count()
    });

    let filtered = use_memo(move || {
        let q = query().trim().to_lowercase();
        let status_now = status();
        let server_now = server();
        let payment_filter = payment();
        let delivery_filter = delivery();
        orders
            .read()
            .iter()
            .filter(|order| {
                let status_ok = status_now == OrderStatus::All || order.status == status_now;
                let payment_ok = payment_filter.matches(&order.method);
                let delivery_ok = delivery_filter.matches(&order.delivery);
                let server_ok = server_now == ALL_SERVERS_KEY || order.server == server_now;
                let search_ok = q.is_empty()
                    || order.player.to_lowercase().contains(&q)
                    || order.product.to_lowercase().contains(&q)
                    || order.email.to_lowercase().contains(&q)
                    || order.id.to_string().contains(&q)
                    || order.method.to_lowercase().contains(&q)
                    || order.server.to_lowercase().contains(&q);
                status_ok && payment_ok && delivery_ok && server_ok && search_ok
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let matched = filtered().len();
    let limit = visible().min(matched);
    let remaining = matched.saturating_sub(limit);
    let can_load_more = remaining > 0;
    let filters_active = status() != OrderStatus::All
        || payment() != PaymentFilter::All
        || delivery() != DeliveryFilter::All
        || server() != ALL_SERVERS_KEY
        || !query().trim().is_empty();

    let selected_order = {
        let filtered_now = filtered();
        selected()
            .and_then(|id| filtered_now.iter().find(|order| order.id == id).cloned())
            .or_else(|| filtered_now.first().cloned())
    };

    rsx! {
        div { class: "mb-6 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("store-orders-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", { t!("store-orders-title") } }
                p { class: "mt-2 text-sm text-text-secondary",
                    { t!("store-orders-sub") }
                }
            }
            if filters_active {
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        query.set(String::new());
                        status.set(OrderStatus::All);
                        payment.set(PaymentFilter::All);
                        delivery.set(DeliveryFilter::All);
                        server.set(ALL_SERVERS_KEY.to_string());
                        selected.set(None);
                        visible.set(ORDER_PAGE_SIZE);
                    },
                    { t!("store-orders-clear-filters") }
                }
            }
        }

        div { class: "store-order-summary mb-5",
            div { class: "store-order-summary-card",
                p { class: "text-xs text-text-muted", { t!("store-orders-showing") } }
                p { class: "mt-1 text-2xl font-semibold tabular-nums", "{limit}" }
                p { class: "mt-0.5 text-xs text-text-muted", { t!("store-orders-of", count: matched) } }
            }
            div { class: "store-order-summary-card",
                p { class: "text-xs text-text-muted", { t!("store-order-status-paid") } }
                p { class: "mt-1 text-2xl font-semibold tabular-nums text-success",
                    "{paid_count()}"
                }
            }
            div { class: "store-order-summary-card",
                p { class: "text-xs text-text-muted", { t!("store-order-status-pending") } }
                p { class: "mt-1 text-2xl font-semibold tabular-nums text-warning",
                    "{pending_count()}"
                }
            }
            div { class: "store-order-summary-card",
                p { class: "text-xs text-text-muted", { t!("store-order-status-refunded") } }
                p {
                    class: "mt-1 text-2xl font-semibold tabular-nums",
                    style: "color: #f87171;",
                    "{refunded_count()}"
                }
            }
        }

        div { class: "store-order-toolbar mb-5",
            SearchInput {
                class: "store-order-search",
                value: query,
                placeholder: t!("store-orders-search-placeholder"),
            }
            div { class: "store-order-filter-row",
                for chip in [OrderStatus::All, OrderStatus::Paid, OrderStatus::Pending, OrderStatus::Refunded] {
                    {
                        let active = status() == chip;
                        rsx! {
                            button {
                                class: if active { "store-type-chip is-active" } else { "store-type-chip" },
                                style: if active { format!("--chip-accent: {}", chip.tone()) } else { String::new() },
                                onclick: move |_| {
                                    status.set(chip);
                                    selected.set(None);
                                },
                                "{chip.label()}"
                            }
                        }
                    }
                }
            }
            div { class: "store-order-selects",
                PaymentFilterSelect { value: payment }
                DeliveryFilterSelect { value: delivery }
                SignalSelect {
                    value: server,
                    options: server_options()
                        .into_iter()
                        .map(|name| {
                            if name == ALL_SERVERS_KEY {
                                SelectOption::new(name.clone(), t_key("store-orders-all-servers"))
                            } else {
                                SelectOption::new(name.clone(), name)
                            }
                        })
                        .collect(),
                }
            }
        }

        div { class: "store-orders-shell",
            div { class: "store-orders-rail",
                if matched == 0 {
                    div { class: "store-orders-empty",
                        p { class: "text-sm font-medium text-text", { t!("store-orders-empty") } }
                        p { class: "mt-1 text-sm text-text-muted",
                            { t!("store-orders-empty-hint") }
                        }
                    }
                } else {
                    for order in filtered().into_iter().take(limit) {
                        {
                            let order_id = order.id;
                            let active = selected_order.as_ref().is_some_and(|item| item.id == order_id);
                            rsx! {
                                button {
                                    key: "{order_id}",
                                    class: if active { "store-order-row is-active" } else { "store-order-row" },
                                    onclick: move |_| selected.set(Some(order_id)),
                                    span { class: "store-order-time", "{order.when}" }
                                    Avatar { email: order.email.clone(), size: 32, alt: order.player.clone() }
                                    div { class: "min-w-0 flex-1",
                                        p { class: "truncate text-sm font-medium text-text", "#{order.id} · {order.product}" }
                                        p { class: "mt-0.5 truncate text-xs text-text-muted", "{order.player} · {order.server}" }
                                    }
                                    div { class: "shrink-0 text-right",
                                        p { class: "text-sm font-semibold tabular-nums", "{order.amount}" }
                                        StatusChip { label: order.status.label(), tone: order.status.tone() }
                                    }
                                }
                            }
                        }
                    }
                    if can_load_more {
                        button {
                            class: "store-orders-load-more",
                            onclick: move |_| {
                                let next = *visible.peek() + ORDER_PAGE_SIZE;
                                visible.set(next.min(matched));
                            },
                            { t!("store-orders-load-more", remaining: remaining) }
                        }
                    }
                }
            }

            aside { class: "store-order-detail",
                if let Some(order) = selected_order {
                    div { class: "flex flex-wrap items-start justify-between gap-3",
                        div {
                            p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                                { t!("store-orders-ticket") }
                            }
                            h2 { class: "mt-2 text-2xl font-semibold tracking-tight",
                                "#{order.id}"
                            }
                            p { class: "mt-1 text-sm text-text-secondary", "{order.product}" }
                        }
                        StatusChip {
                            label: order.status.label(),
                            tone: order.status.tone(),
                        }
                    }
                    div { class: "mt-5 flex items-center gap-3",
                        Avatar {
                            email: order.email.clone(),
                            size: 40,
                            alt: order.player.clone(),
                        }
                        div { class: "min-w-0",
                            p { class: "truncate text-sm font-medium text-text", "{order.player}" }
                        }
                    }
                    div { class: "motion-cascade store-order-detail-grid mt-6",
                        StoreDetailStat { label: t!("store-orders-stat-amount"), value: "{order.amount}" }
                        StoreDetailStat { label: t!("store-orders-stat-placed"), value: t!("store-orders-stat-placed-ago", when: order.when.clone()) }
                        StoreDetailStat { label: t!("store-orders-stat-payment"), value: "{order.method}" }
                        StoreDetailStat { label: t!("store-orders-stat-server"), value: "{order.server}" }
                        StoreDetailStat { label: t!("store-orders-stat-delivery"), value: "{order.delivery}" }
                        StoreDetailStat { label: t!("store-orders-stat-product"), value: "{order.product}" }
                    }
                    div { class: "mt-8 flex flex-wrap gap-2",
                        Button { size: ButtonSize::Sm, { t!("store-orders-refund") } }
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Secondary,
                            { t!("store-orders-resend-delivery") }
                        }
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Ghost,
                            { t!("store-orders-copy-id") }
                        }
                    }
                } else {
                    div { class: "flex h-full min-h-56 flex-col justify-center",
                        p { class: "text-sm font-medium text-text", { t!("store-orders-pick") } }
                        p { class: "mt-1 max-w-xs text-sm text-text-muted",
                            { t!("store-orders-pick-hint") }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StoreDetailStat(#[props(into)] label: String, #[props(into)] value: String) -> Element {
    let _lang = i18n();
    rsx! {
        div {
            p { class: "text-xs text-text-muted", "{label}" }
            p { class: "mt-1 text-sm font-medium text-text", "{value}" }
        }
    }
}

#[component]
fn StoreGatewayRow(
    #[props(into)] name: String,
    #[props(into)] status: String,
    #[props(into)] detail: String,
    connected: bool,
) -> Element {
    let _lang = i18n();
    rsx! {
        div { class: "store-gateway-row",
            div { class: "store-gateway-row-copy min-w-0",
                div { class: "flex flex-wrap items-baseline gap-x-2 gap-y-0.5",
                    h3 { class: "text-sm font-medium text-text", "{name}" }
                    span { class: if connected { "text-xs font-medium text-success" } else { "text-xs font-medium text-text-muted" },
                        "{status}"
                    }
                }
                p { class: "mt-1 text-sm text-text-muted", "{detail}" }
            }
            Button {
                class: "shrink-0 self-start",
                size: ButtonSize::Sm,
                variant: if connected { ButtonVariant::Secondary } else { ButtonVariant::Primary },
                if connected {
                    { t!("store-gateway-manage") }
                } else {
                    { t!("store-gateway-connect") }
                }
            }
        }
    }
}

#[component]
pub fn StoreSettings() -> Element {
    let _lang = i18n();
    let public_path = use_signal(|| String::from("/store"));
    let page_title = use_signal(|| String::from("Store"));
    let nav_labels = use_signal(|| String::from("Shop, Ranks, Crates, Gifts"));

    rsx! {
        FeatureSettingsChrome { subtitle: t!("store-settings-subtitle"),
            DataPanel { title: t!("store-settings-panel-storefront"),
                SettingsControl { label: t!("store-settings-field-public-path"),
                    SignalInput { value: public_path, placeholder: "/store".to_string() }
                }
                SettingsField {
                    label: t!("store-settings-field-full-url"),
                    value: format!("www.example.com{}", public_path()),
                }
                SettingsControl { label: t!("store-settings-field-page-title"),
                    SignalInput { value: page_title, placeholder: "Store".to_string() }
                }
                SettingsControl { label: t!("store-settings-field-nav"),
                    SignalInput {
                        value: nav_labels,
                        placeholder: "Shop, Ranks, Crates, Gifts".to_string(),
                    }
                }
                p { class: "pt-3 text-xs text-text-muted",
                    { t!("store-settings-nav-hint") }
                }
            }
            DataPanel { title: t!("store-settings-panel-gateways"),
                p { class: "py-3 text-sm text-text-muted",
                    { t!("store-settings-gateways-intro") }
                }
                div { class: "motion-cascade motion-cascade-tight store-gateway-list",
                    StoreGatewayRow {
                        name: t_key("store-payment-filter-stripe"),
                        status: t!("store-settings-gateway-live"),
                        detail: t!("store-settings-gateway-stripe-detail"),
                        connected: true,
                    }
                    StoreGatewayRow {
                        name: t_key("store-payment-filter-paypal"),
                        status: t!("store-settings-gateway-sandbox"),
                        detail: t!("store-settings-gateway-paypal-detail"),
                        connected: true,
                    }
                }
            }
            DataPanel { title: t!("store-settings-panel-checkout"),
                SettingRow {
                    title: t!("store-settings-rule-guest-checkout"),
                    description: t!("store-settings-rule-guest-checkout-desc"),
                    enabled: false,
                }
                SettingRow {
                    title: t!("store-settings-rule-save-payment"),
                    description: t!("store-settings-rule-save-payment-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t!("store-settings-rule-test-mode"),
                    description: t!("store-settings-rule-test-mode-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t!("store-settings-rule-receipts"),
                    description: t!("store-settings-rule-receipts-desc"),
                    enabled: true,
                }
            }
            DataPanel { title: t!("store-settings-panel-purchase-rules"),
                SettingRow {
                    title: t!("store-settings-rule-require-login"),
                    description: t!("store-settings-rule-require-login-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t!("store-settings-rule-gift-purchases"),
                    description: t!("store-settings-rule-gift-purchases-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t!("store-settings-rule-creator-codes"),
                    description: t!("store-settings-rule-creator-codes-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t!("store-settings-rule-low-stock"),
                    description: t!("store-settings-rule-low-stock-desc"),
                    enabled: true,
                }
            }
        }
    }
}
