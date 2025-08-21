use super::sortable_table::{truncate_string_at_word, SortableTable};
use crate::main::{Header, Route};
use dioxus::prelude::*;

// The list of languages, similar to your JS array
const LANGUAGES: &[(&str, &str)] = &[
    ["ActionScript", "ActionScript"],
    ["C", "C"],
    ["CSharp", "C#"],
    ["CPP", "C++"],
    ["Clojure", "Clojure"],
    ["CoffeeScript", "CoffeeScript"],
    ["CSS", "CSS"],
    ["Dart", "Dart"],
    ["DM", "DM"],
    ["Elixir", "Elixir"],
    ["Go", "Go"],
    ["Groovy", "Groovy"],
    ["Haskell", "Haskell"],
    ["HTML", "HTML"],
    ["Java", "Java"],
    ["JavaScript", "JavaScript"],
    ["Julia", "Julia"],
    ["Kotlin", "Kotlin"],
    ["Lua", "Lua"],
    ["MATLAB", "MATLAB"],
    ["Objective-C", "Objective-C"],
    ["Perl", "Perl"],
    ["PHP", "PHP"],
    ["PowerShell", "PowerShell"],
    ["Prolog", "Prolog"],
    ["Python", "Python"],
    ["R", "R"],
    ["Ruby", "Ruby"],
    ["Rust", "Rust"],
    ["Scala", "Scala"],
    ["Shell", "Shell"],
    ["Swift", "Swift"],
    ["TeX", "TeX"],
    ["TypeScript", "TypeScript"],
    ["Vim-script", "Vim script"],
];

// The main component for the home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Header { title: "kstars: Top 1000 GitHub Repos per Language".to_string(), show_back_button: false }
        div { class: "container", id: "content",
            for lang_data in LANGUAGES {
                LanguagePreview { language: *lang_data }
            }
        }
    }
}

// A component to show a preview for a single language
#[component]
fn LanguagePreview(language: &'static (&'static str, &'static str)) -> Element {
    // Fetch and parse the top-10 CSV for this language
    let csv_data = use_resource(move || async move {
        let url = format!("/data/processed/top10_{}.csv", language.0);
        fetch_and_parse_csv(&url).await
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

            // Render based on the state of the resource
            match &*csv_data.read() {
                Some(Ok(data)) => {
                    let headers = data.get(0).cloned().unwrap_or_default();
                    let rows: Vec<Vec<String>> = data.iter().skip(1).cloned().collect();
                    rsx!{ SortableTable { headers: headers, rows: rows, truncate: true } }
                }
                Some(Err(e)) => rsx!{ p { "Could not load preview data: {e}" } },
                None => rsx!{ p { "Loading data..." } }
            }
        }
    }
}

// Helper function to fetch and parse CSV data
pub async fn fetch_and_parse_csv(url: &str) -> Result<Vec<Vec<String>>, reqwest::Error> {
    let content = reqwest::get(url).await?.text().await?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(content.as_bytes());

    let records = reader
        .records()
        .filter_map(Result::ok)
        .map(|record| record.iter().map(String::from).collect())
        .collect();

    Ok(records)
}
