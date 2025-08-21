use super::sortable_table::SortableTable;
use crate::{data_loader::get_csv_data, Header, Route};
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
        Header { title: "kstars: Top 1000 GitHub Repos per Language".to_string(), show_back_button: false }
        div { class: "container", id: "content",
            for lang_data in LANGUAGES {
                LanguagePreview { language: lang_data }
            }
        }
    }
}

#[component]
fn LanguagePreview(language: &'static (&'static str, &'static str)) -> Element {
    let csv_data = use_memo(move || {
        let file_name = format!("top10_{}.csv", language.0);
        get_csv_data(&file_name)
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

            if csv_data.read().is_empty() {
                p { "Could not load preview data." }
            } else {
                // --- FIX: Moved the logic directly into the component props ---
                SortableTable {
                    headers: csv_data.read().get(0).cloned().unwrap_or_default(),
                    rows: csv_data.read().iter().skip(1).cloned().collect(),
                    truncate: true
                }
            }
        }
    }
}
