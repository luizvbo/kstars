// src/components/home.rs

use super::sortable_table::SortableTable;
use crate::{data_loader::get_repo_data, Header, Route};
use dioxus::prelude::*;

const LANGUAGES: &[(&str, &str)] = &[
    ("ActionScript", "ActionScript"),
    ("C", "C"),
    ("CSharp", "C#"),
    ("CPP", "C++"),
    ("Clojure", "Clojure"),
    ("CoffeeScript", "CoffeeScript"),
    ("CSS", "CSS"),
    ("Dart", "Dart"),
    ("DM", "DM"),
    ("Elixir", "Elixir"),
    ("Go", "Go"),
    ("Groovy", "Groovy"),
    ("Haskell", "Haskell"),
    ("HTML", "HTML"),
    ("Java", "Java"),
    ("JavaScript", "JavaScript"),
    ("Julia", "Julia"),
    ("Kotlin", "Kotlin"),
    ("Lua", "Lua"),
    ("MATLAB", "MATLAB"),
    ("Objective-C", "Objective-C"),
    ("Perl", "Perl"),
    ("PHP", "PHP"),
    ("PowerShell", "PowerShell"),
    ("Prolog", "Prolog"),
    ("Python", "Python"),
    ("R", "R"),
    ("Ruby", "Ruby"),
    ("Rust", "Rust"),
    ("Scala", "Scala"),
    ("Shell", "Shell"),
    ("Swift", "Swift"),
    ("TeX", "TeX"),
    ("TypeScript", "TypeScript"),
    ("Vim-script", "Vim script"),
];

#[component]
pub fn Home() -> Element {
    rsx! {
        Header { title: "kstars".to_string(), show_back_button: false }

        // --- ADDED: The sticky navigation bar ---
        nav { class: "language-nav",
            for lang_data in LANGUAGES {
                // These are anchor links that jump to the section with the corresponding id
                a { href: "#{lang_data.0}", "{lang_data.1}" }
            }
        }

        div { class: "container", id: "content",
            for lang_data in LANGUAGES {
                LanguagePreview { language: lang_data }
            }
        }
    }
}

#[component]
fn LanguagePreview(language: &'static (&'static str, &'static str)) -> Element {
    let repo_data = use_memo(move || {
        let file_name = format!("top10_{}.csv", language.0);
        get_repo_data(&file_name)
    });

    rsx! {
        div { class: "language-section", id: language.0,
            div { class: "language-header",
                h2 { "{language.1}" }
                Link {
                    to: Route::LanguagePage { lang: language.0.to_string() },
                    class: "cta-link",
                    "View full list (Top 1000)"
                }
            }

            if repo_data.read().is_empty() {
                p { "Could not load preview data." }
            } else {
                // Added a wrapper div for better responsiveness
                div { class: "table-container",
                    SortableTable { repos: repo_data(), truncate: true }
                }
            }
        }
    }
}
