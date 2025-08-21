use crate::THEME;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum SortDirection {
    Ascending,
    Descending,
}

#[component]
pub fn SortableTable(headers: Vec<String>, rows: Vec<Vec<String>>, truncate: bool) -> Element {
    let mut sort_column_index = use_signal::<Option<usize>>(|| None);
    let mut sort_direction = use_signal(|| SortDirection::Ascending);

    let theme_class = if THEME.read().as_str() == "dark" {
        "dark sortable-theme-dark"
    } else {
        "sortable-theme-light"
    };

    let sorted_rows = use_memo(move || {
        let mut sorted = rows.clone();
        if let Some(col_idx) = sort_column_index() {
            sorted.sort_by(|a, b| {
                let val_a = a.get(col_idx).map(|s| s.to_lowercase()).unwrap_or_default();
                let val_b = b.get(col_idx).map(|s| s.to_lowercase()).unwrap_or_default();
                if sort_direction() == SortDirection::Ascending {
                    val_a.cmp(&val_b)
                } else {
                    val_b.cmp(&val_a)
                }
            });
        }
        sorted
    });

    rsx! {
        table {
            "data-sortable": "",
            class: "{theme_class}",
            thead {
                tr {
                    for (i, col) in headers.iter().enumerate() {
                        th {
                            // --- CHANGE ---
                            // The logic is now directly inside the onclick handler.
                            onclick: move |_| {
                                if sort_column_index.read().as_ref() == Some(&i) {
                                    // If clicking the same column, reverse direction
                                    let new_direction = if sort_direction() == SortDirection::Ascending {
                                        SortDirection::Descending
                                    } else {
                                        SortDirection::Ascending
                                    };
                                    sort_direction.set(new_direction);
                                } else {
                                    // If clicking a new column, set it and default to ascending
                                    sort_column_index.set(Some(i));
                                    sort_direction.set(SortDirection::Ascending);
                                }
                            },
                            "{col}"
                            if sort_column_index() == Some(i) {
                                span {
                                    if sort_direction() == SortDirection::Ascending { " ▲" } else { " ▼" }
                                }
                            }
                        }
                    }
                }
            }
            tbody {
                for row in sorted_rows.read().iter() {
                    tr {
                        for cell in row.iter() {
                            td {
                                if truncate {
                                    "{truncate_string_at_word(cell, 150)}"
                                } else {
                                    "{cell}"
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
    if s.len() <= max_chars {
        return s.to_string();
    }

    let mut truncated = s[..max_chars].to_string();
    if let Some(last_space) = truncated.rfind(' ') {
        truncated.truncate(last_space);
    }

    truncated + "..."
}
