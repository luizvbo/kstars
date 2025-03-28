use anyhow::{Context, Result};
use clap::Parser;
use csv::Writer;
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT},
};
use serde::Deserialize;
use std::{fs, path::Path, time::Duration};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Command line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub access token (can be a file path, a string, or read from an environment variable)
    #[arg(short, long, env = "GITHUB_TOKEN")]
    token: Option<String>,

    /// List of languages in the format "api_name:display_name" separated by commas.
    /// Example: "CSharp:C#,CPP:C++" (if display name is omitted, the API name is used)
    #[arg(short, long, value_delimiter = ',')]
    languages: Option<Vec<String>>,

    /// Number of records to retrieve per language (max 1000).
    #[arg(short, long, default_value_t = 1000)]
    records: u32,

    /// Path to folder to store CSV results.
    #[arg(short, long, default_value = "./results")]
    output: String,
}

/// Structure for a GitHub repository (partial data).
#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    html_url: String,
    stargazers_count: u64,
    forks_count: u64,
    watchers_count: u64,
    language: Option<String>,
    description: Option<String>,
    open_issues_count: u64,
    created_at: String,
    pushed_at: String,
    size: u64,
}

/// Structure representing the search API response.
#[derive(Deserialize, Debug)]
struct SearchResponse {
    items: Vec<Repo>,
}

/// Mapping of a language’s API name to its display name.
struct LanguageMapping {
    api_name: String,
    display_name: String,
}

/// Reads the GitHub access token from a file, string, or environment variable.
fn get_access_token(token_input: Option<String>) -> Result<String> {
    if let Some(token) = token_input {
        // Check if it's a valid file path.
        if Path::new(&token).exists() {
            info!("Reading access token from file: {}", token);
            let token = fs::read_to_string(&token)
                .with_context(|| format!("Failed to read access token from file: {}", token))?;
            return Ok(token.trim().to_string());
        }

        // Otherwise, assume it's a direct string.
        info!("Using access token from command-line input.");
        return Ok(token);
    }

    // Fall back to environment variable.
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        info!("Using access token from environment variable.");
        return Ok(token.trim().to_string());
    }

    error!("Access token not provided.");
    anyhow::bail!("Access token not provided.");
}

/// Fetches repositories for a given language and page (each page has 100 results).
/// Fetches repositories for a given language and page (each page has 100 results).
async fn fetch_repos(
    client: &reqwest::Client,
    token: &str,
    language: &str,
    page: u32,
) -> Result<Vec<Repo>> {
    let url = format!(
        "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc&per_page=100&page={}",
        language, page
    );
    debug!("Requesting URL: {}", url);

    // Create a header map and set the required headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-github-app"));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", token)).expect("Invalid token format"),
    );

    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .context("HTTP request failed")?;

    if !resp.status().is_success() {
        error!(
            "Failed to fetch page {} for {}: {}",
            page,
            language,
            resp.status()
        );
        anyhow::bail!("Request failed with status {}", resp.status());
    }

    let search_resp: SearchResponse = resp
        .json()
        .await
        .context("Failed to deserialize JSON response")?;
    debug!(
        "Page {} for {} returned {} repos.",
        page,
        language,
        search_resp.items.len()
    );
    Ok(search_resp.items)
}

/// Fetches up to `records` repositories for the specified language.
/// Iterates in pages of 100 (capped to 10 pages due to GitHub limitations).
async fn fetch_top_repos_for_language(
    client: &Client,
    token: &str,
    language: &str,
    records: u32,
) -> Result<Vec<Repo>> {
    info!("Fetching top repositories for language: {}", language);
    let per_page = 100;
    let pages = ((records + per_page - 1) / per_page).min(10);
    let mut all_repos = Vec::new();

    for page in 1..=pages {
        info!("Fetching page {} for {}", page, language);
        let repos = fetch_repos(client, token, language, page).await?;
        if repos.is_empty() {
            warn!(
                "No repos returned on page {} for {}. Stopping.",
                page, language
            );
            break;
        }
        all_repos.extend(repos);
        // Sleep a bit to help with rate limits.
        sleep(Duration::from_secs(2)).await;
    }
    info!(
        "Total repositories fetched for {}: {}",
        language,
        all_repos.len()
    );
    Ok(all_repos)
}

/// Writes the repository data to a CSV file.
fn write_repos_to_csv<P: AsRef<Path>>(path: P, repos: &[Repo]) -> Result<()> {
    info!(
        "Writing {} repositories to CSV: {:?}",
        repos.len(),
        path.as_ref()
    );
    let mut wtr = Writer::from_path(path)?;
    // Write header.
    wtr.write_record(&[
        "Ranking",
        "Project Name",
        "Stars",
        "Forks",
        "Watchers",
        "Open Issues",
        "Created At",
        "Last Commit",
        "Size (KB)",
        "Description",
        "Language",
        "Repo URL",
    ])?;
    for (i, repo) in repos.iter().enumerate() {
        wtr.write_record(&[
            (i + 1).to_string(),
            repo.name.clone(),
            repo.stargazers_count.to_string(),
            repo.forks_count.to_string(),
            repo.watchers_count.to_string(),
            repo.open_issues_count.to_string(),
            repo.created_at.clone(),
            repo.pushed_at.clone(),
            repo.size.to_string(),
            repo.description.clone().unwrap_or_default(),
            repo.language.clone().unwrap_or_default(),
            repo.html_url.clone(),
        ])?;
    }
    wtr.flush()?;
    info!("CSV file written successfully.");
    Ok(())
}

/// Parses language strings provided from the CLI into LanguageMapping instances.
fn parse_languages(args: Option<Vec<String>>) -> Vec<LanguageMapping> {
    // Default languages if none provided.
    let default = vec![
        ("ActionScript", "ActionScript"),
        ("C", "C"),
        ("CSharp", "C#"),
        ("CPP", "C++"),
        ("Clojure", "Clojure"),
        ("CoffeeScript", "CoffeeScript"),
        ("CSS", "CSS"),
        ("Dart", "Dart"),
        ("DM", "DM"),
        ("Elixir", "Elixir"),
        ("Go", "Go"),
        ("Groovy", "Groovy"),
        ("Haskell", "Haskell"),
        ("HTML", "HTML"),
        ("Java", "Java"),
        ("JavaScript", "JavaScript"),
        ("Julia", "Julia"),
        ("Kotlin", "Kotlin"),
        ("Lua", "Lua"),
        ("MATLAB", "MATLAB"),
        ("Objective-C", "Objective-C"),
        ("Perl", "Perl"),
        ("PHP", "PHP"),
        ("PowerShell", "PowerShell"),
        ("Python", "Python"),
        ("R", "R"),
        ("Ruby", "Ruby"),
        ("Rust", "Rust"),
        ("Scala", "Scala"),
        ("Shell", "Shell"),
        ("Swift", "Swift"),
        ("TeX", "TeX"),
        ("TypeScript", "TypeScript"),
        ("Vim-script", "Vim script"),
    ];

    let mut mappings = Vec::new();
    if let Some(lang_list) = args {
        for lang in lang_list {
            let parts: Vec<&str> = lang.split(':').collect();
            if parts.len() == 2 {
                mappings.push(LanguageMapping {
                    api_name: parts[0].to_string(),
                    display_name: parts[1].to_string(),
                });
            } else {
                mappings.push(LanguageMapping {
                    api_name: lang.clone(),
                    display_name: lang,
                });
            }
        }
    } else {
        for (api, display) in default {
            mappings.push(LanguageMapping {
                api_name: api.to_string(),
                display_name: display.to_string(),
            });
        }
    }
    info!("Parsed {} languages.", mappings.len());
    mappings
}

/// Sets up logging in a uv-inspired style using tracing_subscriber.
///
/// This function configures an environment filter so that RUST_LOG, if set,
/// can override the default. The output is formatted with a simple style.
/// (For more detailed logging including uptime and targets, consider using
/// hierarchical layers as in the uv example.)
fn setup_logging() -> Result<()> {
    // Use an environment filter so that RUST_LOG can override defaults.
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(false)
                .with_timer(fmt::time::UtcTime::rfc_3339()),
        )
        .init();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging.
    setup_logging().context("Failed to set up logging")?;
    info!("Application started.");

    // Parse CLI arguments.
    let args = Args::parse();
    info!("Parsed arguments: {:?}", args);

    // Ensure the output directory exists.
    fs::create_dir_all(&args.output).context("Failed to create output directory")?;
    info!("Output directory ensured at: {}", args.output);

    // Load GitHub token from CLI argument, file, or environment variable.
    let token = get_access_token(args.token)?;
    let client = Client::builder()
        .build()
        .context("Failed to build HTTP client")?;

    // Parse languages.
    let languages = parse_languages(args.languages);

    // For each language, fetch repositories and write CSV.
    for mapping in languages {
        info!(
            "Processing language: {} ({})",
            mapping.display_name, mapping.api_name
        );
        let repos = fetch_top_repos_for_language(&client, &token, &mapping.api_name, args.records)
            .await
            .with_context(|| format!("Failed fetching repos for {}", mapping.api_name))?;

        // Build a safe file name based on display name.
        let safe_name: String = mapping
            .display_name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || vec!['_', '-', '.', '+', '#', ' '].contains(&c) {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        let file_path = format!("{}/{}.csv", args.output, safe_name);
        write_repos_to_csv(&file_path, &repos)
            .with_context(|| format!("Failed writing CSV for {}", mapping.display_name))?;
        info!(
            "Saved {} records for {} in {}",
            repos.len(),
            mapping.display_name,
            file_path
        );
    }

    info!("Application finished successfully.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Repo, parse_languages, write_repos_to_csv};
    use anyhow::Result;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_languages_with_custom_list() {
        let languages = vec![
            "CSharp:C#".to_string(),
            "CPP:C++".to_string(),
            "Python".to_string(),
        ];

        let mappings = parse_languages(Some(languages));

        assert_eq!(mappings.len(), 3);
        assert_eq!(mappings[0].api_name, "CSharp");
        assert_eq!(mappings[0].display_name, "C#");
        assert_eq!(mappings[1].api_name, "CPP");
        assert_eq!(mappings[1].display_name, "C++");
        assert_eq!(mappings[2].api_name, "Python");
        assert_eq!(mappings[2].display_name, "Python");
    }

    #[test]
    fn test_parse_languages_with_default_list() {
        let mappings = parse_languages(None);

        // Check a few key languages from the default list
        assert!(mappings.len() > 10); // Should have many default languages

        // Find a few specific languages
        let rust = mappings.iter().find(|m| m.api_name == "Rust").unwrap();
        let csharp = mappings.iter().find(|m| m.api_name == "CSharp").unwrap();

        assert_eq!(rust.display_name, "Rust");
        assert_eq!(csharp.display_name, "C#");
    }

    #[test]
    fn test_write_repos_to_csv() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("rust.csv");

        let repos = vec![
            Repo {
                name: "rust".to_string(),
                html_url: "https://github.com/rust-lang/rust".to_string(),
                stargazers_count: 50000,
                forks_count: 10000,
                watchers_count: 50000,
                language: Some("Rust".to_string()),
                description: Some("The Rust Programming Language".to_string()),
                open_issues_count: 5000,
                created_at: "2010-01-01T00:00:00Z".to_string(),
                pushed_at: "2023-01-01T00:00:00Z".to_string(),
                size: 100000,
            },
            Repo {
                name: "actix".to_string(),
                html_url: "https://github.com/actix/actix".to_string(),
                stargazers_count: 10000,
                forks_count: 2000,
                watchers_count: 10000,
                language: Some("Rust".to_string()),
                description: Some("Actor framework for Rust".to_string()),
                open_issues_count: 1000,
                created_at: "2018-01-01T00:00:00Z".to_string(),
                pushed_at: "2023-01-02T00:00:00Z".to_string(),
                size: 5000,
            },
        ];

        write_repos_to_csv(&file_path, &repos)?;

        // Check that the file exists
        assert!(file_path.exists());

        // Read the CSV to verify content
        let content = fs::read_to_string(&file_path)?;
        assert!(content.contains("Ranking,Project Name,Stars,Forks"));
        assert!(content.contains("1,rust,50000,10000"));
        assert!(content.contains("2,actix,10000,2000"));

        Ok(())
    }
}
