use anyhow::{Context, Result};
use clap::Parser;
use csv::Writer;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File}, 
    io::{BufReader, BufWriter}, 
    path::{Path, PathBuf}, 
    time::Duration,
};
use serde_json;
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
#[derive(Deserialize, Serialize, Debug, Clone)] 
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

/// Gets the path to the cache directory for a specific language.
fn get_language_cache_dir(output_dir: &str, language_api_name: &str) -> PathBuf {
    PathBuf::from(output_dir)
        .join(".cache") // Store cache in a hidden subfolder
        .join(language_api_name)
}

/// Gets the path to the cache file for a specific page.
fn get_page_cache_file_path(cache_dir: &Path, page: u32) -> PathBuf {
    cache_dir.join(format!("page_{}.json", page))
}

/// Saves a list of repositories for a specific page to its cache file.
fn save_page_to_cache(path: &Path, repos: &[Repo]) -> Result<()> {
    debug!("Saving page cache to: {:?}", path);
    let file = File::create(path)
        .with_context(|| format!("Failed to create cache file: {:?}", path))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, repos)
        .with_context(|| format!("Failed to serialize and write cache file: {:?}", path))?;
    debug!("Page cache saved successfully.");
    Ok(())
}

/// Loads a list of repositories for a specific page from its cache file.
fn load_page_from_cache(path: &Path) -> Result<Vec<Repo>> {
    debug!("Attempting to load page cache from: {:?}", path);
    let file = File::open(path)
        .with_context(|| format!("Failed to open cache file: {:?}", path))?;
    let reader = BufReader::new(file);
    let repos: Vec<Repo> = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to deserialize cache file: {:?}", path))?;
    info!("Loaded {} repos from cache file: {:?}", repos.len(), path);
    Ok(repos)
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

    // Set up headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("rust-github-app"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("token {}", token))
            .expect("Invalid token format"),
    );

    // Send the request
    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .context("HTTP request failed")?;

    // Handle rate limiting if a 403 error is returned
    if resp.status() == reqwest::StatusCode::FORBIDDEN {
        let headers = resp.headers();
        if let Some(retry_after) = headers.get("x-ratelimit-reset") {
            let reset_time: u64 = retry_after.to_str()?.parse()?;
            let wait_time = reset_time - (chrono::Utc::now().timestamp() as u64);
            if wait_time > 0 {
                warn!("Rate limit exceeded. Sleeping for {} seconds...", wait_time);
                tokio::time::sleep(tokio::time::Duration::from_secs(wait_time)).await;
            }
        }
    }

    // Now check if the response was successful
    if !resp.status().is_success() {
        let status = resp.status(); // capture status first
        let error_text = resp
            .text()
            .await
            .unwrap_or_else(|_| "Failed to retrieve error message".to_string());
        error!(
            "Failed to fetch page {} for {}: {}. API message: {}",
            page, language, status, error_text
        );
        anyhow::bail!("Request failed with status {}: {}", status, error_text);
    }

    // Deserialize the response into SearchResponse
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

/// Fetches up to `records` repositories for the specified language, using caching.
/// Iterates in pages of 100 (capped to 10 pages due to GitHub limitations).
async fn fetch_top_repos_for_language(
    client: &Client,
    token: &str,
    language_api_name: &str,
    records: u32,
    output_dir: &str, 
) -> Result<Vec<Repo>> {
    info!("Fetching top repositories for language: {}", language_api_name);
    let per_page = 100;
    // GitHub search API only returns up to 1000 results (10 pages of 100).
    let max_pages = 10;
    let requested_pages = ((records + per_page - 1) / per_page).min(max_pages);
    info!("Planning to fetch {} pages (max {} allowed by API).", requested_pages, max_pages);

    let mut all_repos = Vec::new();

    // Define and ensure the cache directory for this language exists
    let cache_dir = get_language_cache_dir(output_dir, language_api_name);
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("Failed to create cache directory: {:?}", cache_dir))?;
    info!("Using cache directory: {:?}", cache_dir);


    for page in 1..=requested_pages {
        let page_cache_file = get_page_cache_file_path(&cache_dir, page);
        let mut fetched_from_api = false;
        let mut page_repos: Vec<Repo> = Vec::new();

        // Try loading from cache
        if page_cache_file.exists() {
            match load_page_from_cache(&page_cache_file) {
                Ok(repos) => {
                    page_repos = repos;
                }
                Err(e) => {
                    warn!(
                        "Failed to load cache file {:?}: {}. Will attempt to fetch from API.",
                        page_cache_file, e
                    );
                    // Remove the corrupted cache file
                    let _ = fs::remove_file(&page_cache_file);
                }
            }
        }

        // If not loaded from cache, fetch from API
        if page_repos.is_empty() {
             info!("Fetching page {} for {} from API", page, language_api_name);
            match fetch_repos(client, token, language_api_name, page).await {
                Ok(repos) => {
                    if repos.is_empty() && page > 1 { // Check page > 1, as page 1 might genuinely have 0 results
                         warn!(
                            "No repos returned from API on page {} for {}. Stopping.",
                            page, language_api_name
                        );
                        break; // Stop fetching more pages if API returns empty
                    }
                    page_repos = repos;
                    fetched_from_api = true;

                    // 3. Save the newly fetched page to cache
                    if let Err(e) = save_page_to_cache(&page_cache_file, &page_repos) {
                        // Log error but continue, caching isn't critical for the final result
                        error!("Failed to save page {} to cache: {}", page, e);
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to fetch page {} for {}: {}. Stopping processing for this language.",
                        page, language_api_name, e
                    );
                     // Return an error to stop processing this language completely on failure
                    return Err(e).with_context(|| format!("API fetch failed for page {}", page));
                }
            }
        }

        // Add the repos for this page (either from cache or API) to the total
        all_repos.extend(page_repos);

         // Check if we have reached the desired number of records
        if all_repos.len() >= records as usize {
            info!("Reached target of {} records for {}. Stopping fetch.", records, language_api_name);
            // Trim excess records if we fetched a full page but only needed part of it
            all_repos.truncate(records as usize);
            break;
        }

        // Sleep only if we fetched from the API to respect rate limits
        if fetched_from_api {
            debug!("Sleeping for 2 seconds after API call...");
            sleep(Duration::from_secs(2)).await;
        } else {
            // Optional small sleep even for cache hits to avoid overwhelming the disk?
             // sleep(Duration::from_millis(50)).await;
             debug!("Loaded page {} from cache, no API sleep needed.", page);
        }
    }

    info!(
        "Total repositories collected for {}: {}",
        language_api_name,
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
        ("CSharp", "CSharp"),
        ("CPP", "CPP"),
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
        ("Vim-script", "Vim-script"),
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

        // Define cache dir path for potential cleanup
        let cache_dir = get_language_cache_dir(&args.output, &mapping.api_name);

        match fetch_top_repos_for_language(&client, &token, &mapping.api_name, args.records, &args.output)
            .await
        {
            Ok(repos) => {
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
                let safe_name = safe_name.replace(' ', "_"); // Replace spaces for good measure

                let file_path = format!("{}/{}.csv", args.output, safe_name);

                // Write the final combined CSV
                match write_repos_to_csv(&file_path, &repos) {
                    Ok(_) => {
                        info!(
                            "Saved {} records for {} in {}",
                            repos.len(),
                            mapping.display_name,
                            file_path
                        );
                        // Clean up cache directory for this language *only* on success
                        if cache_dir.exists() {
                             info!("Cleaning up cache directory: {:?}", cache_dir);
                            if let Err(e) = fs::remove_dir_all(&cache_dir) {
                                warn!("Failed to remove cache directory {:?}: {}", cache_dir, e);
                            }
                        }
                    }
                    Err(e) => {
                         error!(
                            "Failed writing final CSV for {}: {}. Cache files in {:?} were NOT deleted.",
                            mapping.display_name, e, cache_dir
                        );
                        // Consider how to handle this - maybe return the error from main?
                        // For now, just log it and continue to the next language.
                    }
                }
            }
            Err(e) => {
                 error!(
                    "Failed fetching repos for {}: {}. Skipping this language. Cache files in {:?} may remain.",
                    mapping.api_name, e, cache_dir
                 );
                 // Continue to the next language if one fails
            }
        }
    }

    info!("Application finished processing all requested languages.");
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
