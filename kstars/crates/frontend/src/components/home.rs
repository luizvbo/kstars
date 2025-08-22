use super::sortable_table::SortableTable;
use crate::{data_parser::get_repo_data, LanguageNav, MainHeader, Route};
use dioxus::prelude::*;

pub const LANGUAGES: &[(&str, &str)] = &[
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
        MainHeader { title: "kstars".to_string() }
        LanguageNav {}

        div { class: "container", id: "content",
            for lang_data in LANGUAGES {
                LanguagePreview { language: lang_data }
            }
        }
    }
}

#[component]
fn LanguagePreview(language: &'static (&'static str, &'static str)) -> Element {
    let repo_data_result = use_resource(move || async move {
        let file_name = format!("top10_{}.csv", language.0);
        get_repo_data(&file_name).await
    });

    rsx! {
        div { id: language.0, class: "language-section",
            div { class: "language-header",
                h2 { "{language.1}" }
                Link {
                    to: Route::LanguagePage { lang: language.0.to_string() },
                    class: "cta-link",
                    "View full list (Top 1000)"
                }
            }

            match &*repo_data_result.read() {
                Some(Ok(repos)) => rsx! {
                    div { class: "table-container",
                        SortableTable { repos: repos.clone(), truncate: true }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { style: "padding: 1.5rem; color: red;",
                        h4 { "Error loading preview data" }
                        pre { "{e}" }
                    }
                },
                None => rsx! {
                    p { style: "padding: 1.5rem;", "Loading..." }
                }
            }
        }
    }
}
