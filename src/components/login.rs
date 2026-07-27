use dioxus::prelude::*;

use crate::components::brand::BrandMark;
use crate::components::page::PoweredByFooter;
use crate::components::ui::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "relative flex h-dvh flex-col overflow-y-auto bg-bg text-text",

            div {
                class: "pointer-events-none absolute inset-0",
                style: "background:
                    radial-gradient(ellipse 80% 50% at 50% -10%, rgba(135, 209, 254, 0.14), transparent 55%),
                    radial-gradient(ellipse 60% 40% at 100% 100%, rgba(74, 174, 232, 0.08), transparent 50%),
                    radial-gradient(ellipse 50% 35% at 0% 80%, rgba(135, 209, 254, 0.06), transparent 45%);",
            }

            div {
                class: "relative flex flex-1 flex-col items-center justify-center px-4 py-10",
                div {
                    class: "w-full max-w-sm",

                    div {
                        class: "mb-8 flex flex-col items-center text-center",
                        BrandMark { class: "mb-5 h-12 w-12" }
                        h1 { class: "text-2xl font-semibold tracking-tight", "ServerSpot" }
                        p {
                            class: "mt-2 text-sm text-text-muted",
                            "Sign in to manage your game server website."
                        }
                    }

                    div {
                        class: "flex flex-col gap-4 rounded-squircle-lg border border-border-subtle bg-surface/30 p-5 sm:p-6",

                        div {
                            class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-text-secondary", "Email" }
                            input {
                                r#type: "email",
                                class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                                placeholder: "you@example.com",
                            }
                        }

                        div {
                            class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-text-secondary", "Password" }
                            input {
                                r#type: "password",
                                class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                                placeholder: "••••••••",
                            }
                        }

                        Button {
                            class: "mt-1",
                            full_width: true,
                            "Sign in"
                        }
                    }
                }
            }

            div {
                class: "relative",
                PoweredByFooter {}
            }
        }
    }
}
