use dioxus_logger::tracing;

use crate::schema::Repo;
use std::fmt;

#[derive(Debug)]
pub enum DataError {
    Network(reqwest::Error),
    Message(String),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::Network(err) => write!(f, "Network error: {}", err),
            DataError::Message(msg) => write!(f, "{}", msg),
        }
    }
}

fn parse_repos(csv_content: &str) -> Result<Vec<Repo>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let mut repos = Vec::new();
    let mut errors = Vec::new();

    for (index, result) in reader.deserialize().enumerate() {
        match result {
            Ok(repo) => repos.push(repo),
            Err(e) => {
                errors.push(format!("      - Row {}: {}", index + 2, e));
            }
        }
    }

    if errors.is_empty() {
        Ok(repos)
    } else {
        Err(format!(
            "Failed to parse the CSV file. Found {} error(s):\n{}",
            errors.len(),
            errors.join("\n")
        ))
    }
}

pub async fn get_repo_data(file_name: &str) -> Result<Vec<Repo>, DataError> {
    // --- THIS IS THE CRITICAL FIX, RE-IMPLEMENTED ---
    // 1. Get the current window's origin (e.g., "http://127.0.0.1:8080").
    let origin = web_sys::window()
        .expect("should have a window in this context")
        .location()
        .origin()
        .expect("should have an origin");

    // 2. Construct the full, absolute URL that reqwest needs to work in WASM.
    let url = format!("{}/data/processed/{}", origin, file_name);

    tracing::info!("Fetching data from absolute URL: '{}'", url);

    // 3. Make the request using the full URL.
    let response = reqwest::get(&url).await.map_err(DataError::Network)?;

    if !response.status().is_success() {
        return Err(DataError::Message(format!(
            "Request failed for '{}' with status: {} (Not Found). Check that the file exists in your `public/data/processed` directory.",
            url,
            response.status()
        )));
    }

    let content = response.text().await.map_err(DataError::Network)?;

    // 4. Pass the full content to the parser, which expects a header.
    parse_repos(&content).map_err(DataError::Message)
}
