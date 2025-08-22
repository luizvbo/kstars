mod components;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use gloo_storage::{LocalStorage, Storage};
use components::home::Home;
use components::language_page::LanguagePage;

// --- CHANGES ---
// 1. Made the Route enum public so it can be used in other modules (like home.rs).
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/language/:lang")]
    LanguagePage { lang: String },
}

// 2. Made the THEME signal public.
pub static THEME: GlobalSignal<String> = Signal::global(get_initial_theme);

fn get_initial_theme() -> String {
    LocalStorage::get("theme").unwrap_or_else(|_| "light".to_string())
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    // 3. Apply the theme class to a root div instead of using web_sys on the body.
    // This is more idiomatic and avoids platform-specific code.
    let theme_class = THEME.read().clone();

    rsx! {
        div { class: "{theme_class}",
            link { rel: "stylesheet", href: "/public/style.css" }
            Router::<Route> {}
        }
    }
}

// 4. Made the Header component public.
#[component]
pub fn Header(title: String, show_back_button: bool) -> Element {
    // 5. Use the global THEME signal directly. This is simpler and avoids move/borrow errors.
    let toggle_theme = move |_| {
        let new_theme = if THEME.read().as_str() == "light" {
            "dark"
        } else {
            "light"
        };
        let _ = LocalStorage::set("theme", new_theme);
        THEME.set(new_theme.to_string());
    };

    let theme_icon = if THEME.read().as_str() == "light" {
        "🌙"
    } else {
        "☀️"
    };

    rsx! {
        header {
            h1 { "{title}" }
            div {
                if show_back_button {
                    a { href: "/", style: "color: white; margin-right: 15px", "Back to Home" }
                }
                button { id: "themeToggle", onclick: toggle_theme,
                    span { id: "themeIcon", "{theme_icon}" }
                    " Toggle Theme"
                }
            }
        }
    }
}
