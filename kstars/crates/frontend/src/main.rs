use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

// Import components from other files
mod components;
use components::home::Home;
use components::language_page::LanguagePage;

// Define the routes for the application
#[derive(Routable, Clone, PartialEq, Debug)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/language/:lang")]
    LanguagePage { lang: String },
}

// Global signal for theme management
pub static THEME: GlobalSignal<String> = Signal::global(get_initial_theme);

fn get_initial_theme() -> String {
    use gloo_storage::{LocalStorage, Storage};
    LocalStorage::get("theme").unwrap_or_else(|_| "light".to_string())
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    // Apply the theme class to the body
    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let current_theme = THEME.read().clone();
        if current_theme == "dark" {
            let _ = body.class_list().add_1("dark");
        } else {
            let _ = body.class_list().remove_1("dark");
        }
    });

    rsx! {
        // Link to the stylesheet
        link { rel: "stylesheet", href: "/public/style.css" }

        // The router manages which page is currently displayed
        Router::<Route> {}
    }
}

// A reusable header component with the theme toggle button
#[component]
pub fn Header(title: String, show_back_button: bool) -> Element {
    let mut theme = THEME;

    let toggle_theme = move |_| {
        use gloo_storage::{LocalStorage, Storage};
        let new_theme = if theme.read().as_str() == "light" {
            "dark"
        } else {
            "light"
        };
        let _ = LocalStorage::set("theme", new_theme);
        theme.set(new_theme.to_string());
    };

    let theme_icon = if theme.read().as_str() == "light" {
        "🌙"
    } else {
        "☀️"
    };

    rsx! {
        header {
            h1 { "{title}" }
            div {
                if show_back_button {
                    a {
                        href: "/",
                        style: "color: white; margin-right: 15px",
                        "Back to Home"
                    }
                }
                button { id: "themeToggle", onclick: toggle_theme,
                    span { id: "themeIcon", "{theme_icon}" }
                    " Toggle Theme"
                }
            }
        }
    }
}
