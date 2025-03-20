use anyhow::{Context, Result};
use clap::Parser;
use csv::Writer;
use reqwest::Client;
use serde::Deserialize;
use std::{fs, path::Path, time::Duration};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Command line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
    fork_count: Option<u64>,
    language: Option<String>,
    description: Option<String>,
    open_issues_count: u64,
    pushed_at: String,
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

/// Reads the GitHub access token from "access_token.txt".
fn get_access_token() -> Result<String> {
    let token = fs::read_to_string("access_token.txt")
        .context("Failed to read access_token.txt")?
        .trim()
        .to_string();
    info!("Access token loaded successfully.");
    Ok(token)
}

/// Fetches repositories for a given language and page (each page has 100 results).
async fn fetch_repos(client: &Client, token: &str, language: &str, page: u32) -> Result<Vec<Repo>> {
    let url = format!(
        "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc&per_page=100&page={}",
        language, page
    );
    debug!("Requesting URL: {}", url);
    let resp = client
        .get(&url)
        .header("User-Agent", "rust-github-app")
        .header("Authorization", format!("token {}", token))
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
        "Language",
        "Repo URL",
        "Username",
        "Open Issues",
        "Last Commit",
        "Description",
    ])?;
    for (i, repo) in repos.iter().enumerate() {
        wtr.write_record(&[
            (i + 1).to_string(),
            repo.name.clone(),
            repo.stargazers_count.to_string(),
            repo.fork_count.unwrap_or(0).to_string(), // Use 0 if fork_count is None
            repo.language.clone().unwrap_or_default(),
            repo.html_url.clone(),
            "".to_string(), // Username not available in this endpoint.
            repo.open_issues_count.to_string(),
            repo.pushed_at.clone(),
            repo.description.clone().unwrap_or_default(),
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
        //("C", "C"),
        //("CSharp", "C#"),
        //("CPP", "C++"),
        //("Clojure", "Clojure"),
        //("CoffeeScript", "CoffeeScript"),
        //("CSS", "CSS"),
        //("Dart", "Dart"),
        //("DM", "DM"),
        //("Elixir", "Elixir"),
        //("Go", "Go"),
        //("Groovy", "Groovy"),
        //("Haskell", "Haskell"),
        //("HTML", "HTML"),
        //("Java", "Java"),
        //("JavaScript", "JavaScript"),
        //("Julia", "Julia"),
        //("Kotlin", "Kotlin"),
        //("Lua", "Lua"),
        //("MATLAB", "MATLAB"),
        //("Objective-C", "Objective-C"),
        //("Perl", "Perl"),
        //("PHP", "PHP"),
        //("PowerShell", "PowerShell"),
        //("Python", "Python"),
        //("R", "R"),
        //("Ruby", "Ruby"),
        //("Rust", "Rust"),
        //("Scala", "Scala"),
        //("Shell", "Shell"),
        //("Swift", "Swift"),
        //("TeX", "TeX"),
        //("TypeScript", "TypeScript"),
        //("Vim-script", "Vim script"),
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

    // Load GitHub token.
    let token = get_access_token()?;
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
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
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
