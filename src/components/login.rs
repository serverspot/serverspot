use dioxus::prelude::*;

use crate::components::brand::BrandMark;
use crate::components::page::PoweredByFooter;
use crate::components::ui::*;
use crate::router::Route;

#[component]
pub fn Login() -> Element {
    let navigator = use_navigator();
    let mut show_password = use_signal(|| false);

    rsx! {
        div { class: "login-page",
            div { class: "login-atmosphere", "aria-hidden": "true" }
            div { class: "login-grid", "aria-hidden": "true" }

            div { class: "login-stage",

                header { class: "login-hero",
                    BrandMark { class: "login-mark" }
                    h1 { class: "login-brand-name", "ServerSpot" }
                    p { class: "login-hero-line", "Admin panel" }
                }

                form {
                    class: "login-form",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        navigator.push(Route::Dashboard {});
                    },

                    div { class: "login-field",
                        label { class: "login-label", r#for: "login-email", "Email" }
                        input {
                            id: "login-email",
                            r#type: "email",
                            autocomplete: "email",
                            class: "login-input",
                            placeholder: "you@novacraft.gg",
                        }
                    }

                    div { class: "login-field",
                        div { class: "login-field-top",
                            label { class: "login-label", r#for: "login-password", "Password" }
                            button { r#type: "button", class: "login-link", "Forgot?" }
                        }
                        div { class: "login-password",
                            input {
                                id: "login-password",
                                r#type: if show_password() { "text" } else { "password" },
                                autocomplete: "current-password",
                                class: "login-input",
                                placeholder: "••••••••",
                            }
                            button {
                                r#type: "button",
                                class: "login-reveal",
                                "aria-label": if show_password() { "Hide password" } else { "Show password" },
                                onclick: move |_| show_password.set(!show_password()),
                                if show_password() {
                                    "Hide"
                                } else {
                                    "Show"
                                }
                            }
                        }
                    }

                    label { class: "login-remember",
                        input { r#type: "checkbox", class: "login-checkbox" }
                        span { "Keep me signed in" }
                    }

                    Button {
                        full_width: true,
                        onclick: move |_| {
                            navigator.push(Route::Dashboard {});
                        },
                        "Sign in"
                    }

                    div { class: "login-divider", "or" }

                    button { r#type: "button", class: "login-oauth",
                        DiscordMark {}
                        "Continue with Discord"
                    }
                }

                p { class: "login-foot", "Need access? Ask your server owner for an invite." }
            }

            PoweredByFooter {}
        }
    }
}

#[component]
fn DiscordMark() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            "aria-hidden": "true",
            path { d: "M19.27 5.33C17.94 4.71 16.5 4.26 15 4a.09.09 0 0 0-.07.03c-.18.33-.39.76-.53 1.09a16.09 16.09 0 0 0-4.8 0c-.14-.34-.37-.76-.54-1.09c-.01-.02-.04-.03-.07-.03c-1.5.26-2.93.71-4.27 1.33c-.01 0-.02.01-.03.02c-2.72 4.07-3.47 8.03-3.1 11.95c0 .02.01.04.03.05c1.8 1.32 3.53 2.12 5.24 2.65c.03.01.06 0 .07-.02c.4-.55.76-1.13 1.07-1.74c.02-.04 0-.08-.04-.09c-.57-.22-1.11-.48-1.64-.78c-.04-.02-.04-.08-.01-.11c.11-.08.22-.17.33-.25c.02-.02.05-.02.07-.01c3.44 1.57 7.15 1.57 10.55 0c.02-.01.05-.01.07.01c.11.09.22.17.33.26c.04.03.04.09-.01.11c-.52.31-1.07.56-1.64.78c-.04.01-.05.06-.04.09c.32.61.68 1.19 1.07 1.74c.03.02.06.03.09.02c1.72-.53 3.45-1.33 5.25-2.65c.02-.01.03-.03.03-.05c.44-4.53-.73-8.46-3.1-11.95c-.01-.01-.02-.02-.04-.02zM8.52 14.91c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.84 2.12-1.89 2.12zm6.97 0c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.83 2.12-1.89 2.12z" }
        }
    }
}
