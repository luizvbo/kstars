// src/data_parser.rs

use crate::schema::Repo;
use std::fmt;
use web_sys::console; // Import the console for guaranteed logging

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

// This function correctly assumes the CSV content has a header row.
fn parse_repos(csv_content: &str) -> Result<Vec<Repo>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true) // CRITICAL: Tell the parser the first row is the header.
        .from_reader(csv_content.as_bytes());

    let mut repos = Vec::new();
    let mut errors = Vec::new();

    for (index, result) in reader.deserialize().enumerate() {
        match result {
            Ok(repo) => repos.push(repo),
            Err(e) => {
                // Collect any real data errors (e.g., a row with wrong column count).
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
    let origin = web_sys::window()
        .expect("should have a window")
        .location()
        .origin()
        .expect("should have an origin");

    let url = format!("{}/data/processed/{}", origin, file_name);
    
    // --- GUARANTEED LOGGING ---
    // This uses the browser's console.log directly. It cannot fail to appear.
    console::log_1(&format!("Fetching data from: {}", url).into());

    let response = reqwest::get(&url).await.map_err(DataError::Network)?;

    if !response.status().is_success() {
        return Err(DataError::Message(format!(
            "Request failed for '{}' with status: {} (Not Found). Check that the file exists in your `data/processed` directory and that the server is running correctly.",
            url,
            response.status()
        )));
    }

    // Get the raw text content.
    let content = response.text().await.map_err(DataError::Network)?;

    // --- THE FIX ---
    // We no longer manually process the string. We pass the full, raw content
    // directly to the parser, which is configured to handle the header row itself.
    parse_repos(&content).map_err(DataError::Message)
}
