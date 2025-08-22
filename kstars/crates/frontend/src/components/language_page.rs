use super::sortable_table::SortableTable;
use crate::{data_loader::get_repo_data, MainHeader};
use dioxus::prelude::*;

#[component]
pub fn LanguagePage(lang: String) -> Element {
    let page_title = format!("kstars: Top 1000 GitHub Repos for {}", lang);
    let header_title = lang.clone();
    let lang_for_memo = lang.clone();

    let repo_data = use_memo(move || {
        let file_name = format!("{}.csv", lang_for_memo);
        get_repo_data(&file_name)
    });

    rsx! {
        document::Title { "{page_title}" }
        MainHeader { title: "kstars".to_string() }

        div { class: "container", id: "language-content",
            div { class: "language-header",
                h2 { "{header_title}" }
                Link { to: "/", class: "cta-link", "← Back to all languages" }
            }
            if repo_data().is_empty() {
                p { "No repository data found for {lang}." }
            } else {
                div { class: "table-container",
                    SortableTable { repos: repo_data(), truncate: true }
                }
            }
        }
    }
}
