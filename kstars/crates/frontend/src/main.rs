use anyhow::{anyhow, Context, Result};
use dioxus::prelude::*;
use gloo_net::http::Request;

#[derive(Clone, Debug, PartialEq)]
struct NamedTable {
    name: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

pub fn main() {
    // The renderer is selected by features; here we enabled "web" in Cargo.toml.
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Reactive state
    let mut tables: Signal<Vec<NamedTable>> = use_signal(Vec::new);
    let mut url_input = use_signal(String::new);
    let mut error_text: Signal<Option<String>> = use_signal(|| None);

    // Minimal CSS (feel free to move to a stylesheet/asset)
    const STYLE: &str = r#"
    :root { font-family: system-ui, Segoe UI, Roboto, sans-serif; color-scheme: light dark; }
    body { margin: 2rem; }
    .controls { display: grid; gap: 0.75rem; max-width: 920px; margin-bottom: 1.25rem; }
    .bar { display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap; }
    input[type='text'] { flex: 1 1 420px; padding: 0.5rem; }
    button { padding: 0.5rem 0.8rem; cursor: pointer; }
    .card { margin: 1rem 0; padding: 0.75rem 1rem; border: 1px solid #8884; border-radius: 8px; }
    .filename { font-weight: 600; margin-bottom: 0.5rem; }
    .table-wrap { overflow: auto; max-width: 100%; border: 1px solid #8884; border-radius: 6px; }
    table { border-collapse: collapse; width: 100%; font-size: 0.95rem; }
    th, td { border-bottom: 1px solid #8883; padding: 0.45rem 0.55rem; text-align: left; white-space: nowrap; }
    thead th { position: sticky; top: 0; background: color-mix(in oklab, Canvas, CanvasText 5%); }
    tbody tr:nth-child(odd) td { background: color-mix(in oklab, Canvas, CanvasText 3%); }
    .muted { opacity: 0.8; }
    .error { color: #b00020; font-weight: 600; }
    "#;

    rsx! {
        document::Style { "{STYLE}" }
        h1 { "CSV → Tables (Dioxus • Rust • WASM)" }

        // --- Controls -------------------------------------------------------
        div { class: "controls",
            // File picker: local CSVs (multi-file)
            div { class: "bar",
                label { r#for: "csv_files", class: "muted", "Pick CSV files:" }
                input {
                    id: "csv_files",
                    r#type: "file",
                    accept: ".csv,text/csv",
                    multiple: true,
                    // Folder selection (supported in modern browsers)
                    directory: true,
                    // Read files asynchronously (Dioxus supports async event handlers)
                    // See docs & example: evt.files() -> FileEngine; read_file_to_string(...).await
                    onchange: move |evt| async move {
                        if let Some(file_engine) = evt.files() {
                            let file_names = file_engine.files();
                            for name in &file_names {
                                if let Some(text) = file_engine.read_file_to_string(name).await {
                                    match parse_csv_to_table(&text, name.clone()) {
                                        Ok(tbl) => tables.write().push(tbl),
                                        Err(e) => error_text.set(Some(format!("{}: {e}", name))),
                                    }
                                } else {
                                    error_text.set(Some(format!("Failed to read file: {name}")));
                                }
                            }
                        }
                    },
                }
                button {
                    onclick: move |_| {
                        tables.write().clear();
                        error_text.set(None);
                    },
                    "Clear tables"
                }
            }

            // URL loader: fetch CSV by URL
            div { class: "bar",
                label { r#for: "csv_url", class: "muted", "…or load by URL:" }
                input {
                    id: "csv_url",
                    r#type: "text",
                    placeholder: "https://example.com/data.csv",
                    value: "{url_input}",
                    oninput: move |e| url_input.set(e.value()),
                }
                button {
                    onclick: move |_| {
                        let url = url_input();
                        if url.trim().is_empty() {
                            error_text.set(Some("Enter a CSV URL first".into()));
                            return;
                        }
                        let mut tables = tables.clone();
                        let mut error_text = error_text.clone();
                        spawn(async move {
                            match load_csv_from_url(&url).await {
                                Ok(tbl) => {
                                    tables.write().push(tbl);
                                    error_text.set(None);
                                }
                                Err(e) => error_text.set(Some(format!("URL load failed: {e}"))),
                            }
                        });
                    },
                    "Load CSV"
                }
            }

            {error_text().as_ref().map(|msg| rsx! {
                p { class: "error", "{msg}" }
            })}
                // Error banner
        }

        // --- Render every loaded table -------------------------------------
        for t in tables().iter() {
            TableView { table: t.clone() }
        }

        // Footer tip about CORS for URL fetches
        p { class: "muted",
            "Tip: Loading CSV by URL must satisfy the browser’s CORS rules (server should include "
            code { "Access-Control-Allow-Origin" }
            ")."
        }
    }
}

// Render a single table
#[component]
fn TableView(table: NamedTable) -> Element {
    rsx! {
        div { class: "card",
            div { class: "filename", "{table.name}" }
            div { class: "table-wrap",
                table {
                    thead {
                        tr {
                            for h in &table.headers {
                                th { "{h}" }
                            }
                        }
                    }
                    tbody {
                        for row in &table.rows {
                            tr {
                                for cell in row {
                                    td { "{cell}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn load_csv_from_url(url: &str) -> Result<NamedTable> {
    let text = Request::get(url)
        .send()
        .await
        .with_context(|| format!("GET {url}"))?
        .text()
        .await
        .with_context(|| format!("read body from {url}"))?;

    // Use last path segment as a friendly name
    let name = url.rsplit('/').next().unwrap_or(url).to_string();
    parse_csv_to_table(&text, name)
}

/// Parse CSV text to a NamedTable
fn parse_csv_to_table(text: &str, name: String) -> Result<NamedTable> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true) // tolerate ragged rows
        .from_reader(text.as_bytes());

    let headers = rdr
        .headers()
        .map_err(|e| anyhow!("header parse error: {e}"))?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut rows: Vec<Vec<String>> = Vec::new();
    for rec in rdr.records() {
        let r = rec.map_err(|e| anyhow!("row parse error: {e}"))?;
        let mut v = r.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        // Pad/truncate to header width for a tidy table
        if v.len() < headers.len() {
            v.resize(headers.len(), String::new());
        } else if v.len() > headers.len() {
            v.truncate(headers.len());
        }
        rows.push(v);
    }

    Ok(NamedTable {
        name,
        headers,
        rows,
    })
}
