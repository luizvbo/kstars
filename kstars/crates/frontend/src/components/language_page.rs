// src/components/language_page.rs

use super::home::fetch_and_parse_csv;
use super::sortable_table::SortableTable;
// --- CHANGE ---
// Corrected the import path.
use crate::Header;
use dioxus::prelude::*;

#[component]
pub fn LanguagePage(lang: String) -> Element {
    let page_title = format!("kstars: Top 1000 GitHub Repos for {}", lang);
    let header_title = format!("Top 1000 GitHub Repos for {}", lang);

    let csv_data = use_resource(move || {
        let lang_clone = lang.clone();
        async move {
            let url = format!("/data/processed/{}.csv", lang_clone);
            fetch_and_parse_csv(&url).await
        }
    });

    rsx! {
        // --- CHANGE ---
        // Used the full path `document::Title` to bring the component into scope.
        document::Title { "{page_title}" }
        Header { title: header_title, show_back_button: true }
        
        div { class: "container", id: "language-content",
            match &*csv_data.read() {
                Some(Ok(data)) => {
                    let headers = data.get(0).cloned().unwrap_or_default();
                    let rows: Vec<Vec<String>> = data.iter().skip(1).cloned().collect();
                    rsx!{ SortableTable { headers: headers, rows: rows, truncate: true } }
                }
                Some(Err(e)) => rsx!{ p { "Error loading repository data: {e}" } },
                None => rsx!{ p { id: "loading-message", "Loading data..." } }
            }
        }
    }
}
