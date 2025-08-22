// src/components/language_page.rs

use super::sortable_table::SortableTable;
use crate::{data_loader::get_repo_data, Header};
use dioxus::prelude::*;

#[component]
pub fn LanguagePage(lang: String) -> Element {
    let page_title = format!("kstars: Top 1000 GitHub Repos for {}", lang);
    let header_title = format!("Top 1000 GitHub Repos for {}", lang);
    let lang_for_memo = lang.clone();

    let repo_data = use_memo(move || {
        let file_name = format!("{}.csv", lang_for_memo);
        get_repo_data(&file_name)
    });

    rsx! {
        document::Title { "{page_title}" }
        Header { title: header_title, show_back_button: true }

        div { class: "container", id: "language-content",
            // The new CSS will apply to this section automatically
            div { class: "language-section",
                 div { class: "language-header",
                    h2 { "Top 1000 Repositories" }
                }
                if repo_data.read().is_empty() {
                    p { id: "loading-message", "No repository data found for {lang}." }
                } else {
                    // Added a wrapper div for better responsiveness
                    div { class: "table-container",
                        SortableTable { repos: repo_data(), truncate: true }
                    }
                }
            }
        }
    }
}
