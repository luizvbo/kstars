use super::sortable_table::SortableTable;
use crate::{data_loader::get_csv_data, Header};
use dioxus::prelude::*;

#[component]
pub fn LanguagePage(lang: String) -> Element {
    let page_title = format!("kstars: Top 1000 GitHub Repos for {}", lang);
    let header_title = format!("Top 1000 GitHub Repos for {}", lang);

    // --- FIX: Clone `lang` before moving it into the closure ---
    // We create a new variable that the closure can take ownership of.
    let lang_for_memo = lang.clone();

    let csv_data = use_memo(move || {
        // The closure now moves `lang_for_memo`, not the original `lang`.
        let file_name = format!("{}.csv", lang_for_memo);
        get_csv_data(&file_name)
    });

    rsx! {
        document::Title { "{page_title}" }
        Header { title: header_title, show_back_button: true }

        div { class: "container", id: "language-content",
            if csv_data.read().is_empty() {
                // The original `lang` is still valid and can be used here.
                p { id: "loading-message", "No repository data found for {lang}." }
            } else {
                SortableTable {
                    headers: csv_data.read().get(0).cloned().unwrap_or_default(),
                    rows: csv_data.read().iter().skip(1).cloned().collect(),
                    truncate: true
                }
            }
        }
    }
}
