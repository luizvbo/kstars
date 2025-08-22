// src/main.rs

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use gloo_storage::{LocalStorage, Storage};

mod components;
mod data_parser;
mod schema;

use components::home::{Home, LANGUAGES};
use components::language_page::LanguagePage;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/language/:lang")]
    LanguagePage { lang: String },
}

pub static THEME: GlobalSignal<String> = Signal::global(get_initial_theme);
// const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS_FILE: Asset = asset!("/assets/css/style.css");

fn get_initial_theme() -> String {
    LocalStorage::get("theme").unwrap_or_else(|_| "light".to_string())
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let current_theme = THEME.read().clone();
                    if current_theme == "dark" {
                        let _ = body.class_list().add_1("dark");
                    } else {
                        let _ = body.class_list().remove_1("dark").ok();
                    }
                }
            }
        }
    });

    rsx! {
        link { rel: "stylesheet", href: CSS_FILE }
        Router::<Route> {}
    }
}

#[component]
pub fn Header(title: String, show_back_button: bool, is_home: bool) -> Element {
    let toggle_theme = move |_| {
        let new_theme = if THEME.read().as_str() == "light" {
            "dark"
        } else {
            "light"
        };
        let _ = LocalStorage::set("theme", new_theme);
        *THEME.write() = new_theme.to_string();
    };

    let theme_icon = if THEME.read().as_str() == "light" {
        "🌙"
    } else {
        "☀️"
    };

    rsx! {
        header {
            div { class: "header-content",
                h1 { "{title}" }
                nav { class: "header-nav",
                    if is_home {
                        div { class: "language-nav-links",
                            for lang_data in LANGUAGES {
                                a { href: "#{lang_data.0}", "{lang_data.1}" }
                            }
                        }
                    }
                    if show_back_button {
                        // --- CHANGE: Made this a button-like link ---
                        Link { to: Route::Home {}, class: "cta-link",
                            button { "Back to Home" }
                        }
                    }
                    button { onclick: toggle_theme, "{theme_icon}" }
                }
            }
        }
    }
}

#[component]
pub fn MainHeader(title: String) -> Element {
    let toggle_theme = move |_| {
        let new_theme = if THEME.read().as_str() == "light" { "dark" } else { "light" };
        let _ = LocalStorage::set("theme", new_theme);
        *THEME.write() = new_theme.to_string();
    };
    let theme_icon = if THEME.read().as_str() == "light" { "🌙" } else { "☀️" };

    rsx! {
        header { class: "main-header",
            div { class: "header-content",
                h1 { "{title}" }
                button { onclick: toggle_theme, "{theme_icon}" }
            }
        }
    }
}

#[component]
pub fn LanguageNav() -> Element {
    rsx! {
        nav { class: "language-nav",
            div { class: "language-nav-links",
                for lang_data in LANGUAGES {
                    a { href: "#{lang_data.0}", "{lang_data.1}" }
                }
            }
        }
    }
}
