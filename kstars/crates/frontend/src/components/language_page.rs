use super::sortable_table::SortableTable;
use crate::{data_parser::get_repo_data, MainHeader};
use dioxus::prelude::*;

#[component]
pub fn LanguagePage(lang: String) -> Element {
    let page_title = format!("kstars: Top 1000 GitHub Repos for {}", lang);
    let header_title = lang.clone();
    let lang_for_resource = lang.clone();

    let repo_data_result = use_resource(move || {
        let lang_clone = lang_for_resource.clone();
        async move { get_repo_data(&format!("{}.csv", lang_clone)).await }
    });

    rsx! {
        document::Title { "{page_title}" }
        MainHeader { title: "kstars".to_string() }

        div { class: "container", id: "language-content",
            div { class: "language-header",
                h2 { "{header_title}" }
                Link { to: "/", class: "cta-link", "← Back to all languages" }
            }
            match &*repo_data_result.read() {
                Some(Ok(repos)) => rsx! {
                    div { class: "table-container",
                        SortableTable { repos: repos.clone(), truncate: true }
                    }
                },
                Some(Err(e)) => rsx! {
                     div { style: "padding: 1.5rem; color: red;",
                        h4 { "Error loading data for {lang}" }
                        pre { "{e}" }
                    }
                },
                None => rsx! {
                    p { "Loading..." }
                }
            }
        }
    }
}
