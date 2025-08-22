// src/components/sortable_table.rs

use crate::{schema::Repo, THEME};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum SortDirection {
    Ascending,
    Descending,
}

const HEADERS: &[&str] = &[
    "Ranking",
    "Project Name",
    "Stars",
    "Forks",
    "Watchers",
    "Open Issues",
    "Created At",
    "Last Commit",
    "Size",
    "Size (KB)",
    "Description",
    "Language",
    "Repo URL",
];

#[component]
pub fn SortableTable(repos: Vec<Repo>, truncate: bool) -> Element {
    let mut sort_column = use_signal::<Option<&'static str>>(|| None);
    let mut sort_direction = use_signal(|| SortDirection::Ascending);

    // --- FIX: Removed the local `theme` variable ---

    let sorted_repos = use_memo(move || {
        let mut sorted = repos.clone();
        if let Some(column) = sort_column() {
            sorted.sort_by(|a, b| {
                let ordering = match column {
                    "Ranking" => a.ranking.cmp(&b.ranking),
                    "Project Name" => a.project_name.cmp(&b.project_name),
                    "Stars" => a.stars.cmp(&b.stars),
                    "Forks" => a.forks.cmp(&b.forks),
                    "Watchers" => a.watchers.cmp(&b.watchers),
                    "Open Issues" => a.open_issues.cmp(&b.open_issues),
                    "Size (KB)" => a.size_kb.cmp(&b.size_kb),
                    _ => a
                        .project_name
                        .to_lowercase()
                        .cmp(&b.project_name.to_lowercase()),
                };

                if sort_direction() == SortDirection::Descending {
                    ordering.reverse()
                } else {
                    ordering
                }
            });
        }
        sorted
    });

    rsx! {
        table {
            "data-sortable": "",
            // --- FIX: Use the global THEME signal directly ---
            class: if THEME.read().as_str() == "dark" { "dark sortable-theme-dark" } else { "sortable-theme-light" },
            thead {
                tr {
                    for &header in HEADERS {
                        th {
                            onclick: move |_| {
                                if sort_column.read().as_ref() == Some(&header) {
                                    let new_dir = if sort_direction() == SortDirection::Ascending {
                                        SortDirection::Descending
                                    } else {
                                        SortDirection::Ascending
                                    };
                                    sort_direction.set(new_dir);
                                } else {
                                    sort_column.set(Some(header));
                                    sort_direction.set(SortDirection::Ascending);
                                }
                            },
                            "{header}"
                            if sort_column() == Some(header) {
                                span {
                                    if sort_direction() == SortDirection::Ascending { " ▲" } else { " ▼" }
                                }
                            }
                        }
                    }
                }
            }
            tbody {
                for repo in sorted_repos.read().iter() {
                    tr {
                        td { "{repo.ranking}" }
                        td { "{repo.project_name}" }
                        td { "{repo.stars}" }
                        td { "{repo.forks}" }
                        td { "{repo.watchers}" }
                        td { "{repo.open_issues}" }
                        td { "{repo.created_at}" }
                        td { "{repo.last_commit}" }
                        td { "{repo.size}" }
                        td { "{repo.size_kb}" }
                        td {
                            if truncate {
                                "{truncate_string_at_word(&repo.description, 150)}"
                            } else {
                                "{repo.description}"
                            }
                        }
                        td { "{repo.language}" }
                        td {
                            {
                                let display_url = repo.repo_url
                                    .strip_prefix("https://github.com/")
                                    .unwrap_or(&repo.repo_url);
                                rsx! {
                                    a { href: "{repo.repo_url}", target: "_blank", "{display_url}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn truncate_string_at_word(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let end_byte_index = s
        .char_indices()
        .nth(max_chars)
        .map(|(idx, _)| idx)
        .unwrap_or(s.len());
    let mut truncated = s[..end_byte_index].to_string();
    if let Some(last_space) = truncated.rfind(' ') {
        truncated.truncate(last_space);
    }
    truncated + "..."
}
