use clap::Parser;
use csv::Writer;
use reqwest::Client;
use serde::Deserialize;
use std::{
    error::Error,
    fs,
    path::Path,
    time::Duration,
};
use tokio::time::sleep;

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
    fork_count: u64,
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
fn get_access_token() -> Result<String, Box<dyn Error>> {
    let token = fs::read_to_string("access_token.txt")?.trim().to_string();
    Ok(token)
}

/// Fetches repositories for a given language and page.
/// Uses per_page=100.
async fn fetch_repos(
    client: &Client,
    token: &str,
    language: &str,
    page: u32,
) -> Result<Vec<Repo>, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc&per_page=100&page={}",
        language, page
    );
    let resp = client
        .get(&url)
        .header("User-Agent", "rust-github-app")
        .header("Authorization", format!("token {}", token))
        .send()
        .await?;
    
    if !resp.status().is_success() {
        Err(format!(
            "Failed to fetch page {} for {}: {}",
            page, language, resp.status()
        ))?
    } else {
        let search_resp: SearchResponse = resp.json().await?;
        Ok(search_resp.items)
    }
}

/// Fetches up to `records` repositories for the specified language.
/// Since the API is capped at 1000 results, we iterate in pages of 100 (up to 10 pages).
async fn fetch_top_repos_for_language(
    client: &Client,
    token: &str,
    language: &str,
    records: u32,
) -> Result<Vec<Repo>, Box<dyn Error>> {
    println!("Fetching top repositories for language: {}", language);
    let per_page = 100;
    let pages = (records + per_page - 1) / per_page;
    let pages = pages.min(10); // GitHub search API caps at 1000 results.
    let mut all_repos = Vec::new();

    for page in 1..=pages {
        let repos = fetch_repos(client, token, language, page).await?;
        if repos.is_empty() {
            break;
        }
        println!("Page {}: Fetched {} repositories", page, repos.len());
        all_repos.extend(repos);
        // Pause to respect GitHub rate limits.
        sleep(Duration::from_secs(2)).await;
    }
    println!(
        "Total repositories fetched for {}: {}\n",
        language,
        all_repos.len()
    );
    Ok(all_repos)
}

/// Write the repositories data into a CSV file.
fn write_repos_to_csv<P: AsRef<Path>>(path: P, repos: &[Repo]) -> Result<(), Box<dyn Error>> {
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
        // Extract owner from the URL if needed; here we keep it blank as the REST API
        // doesn't provide owner login separately in this endpoint.
        // Alternatively, you might parse the URL or use GraphQL for more details.
        wtr.write_record(&[
            (i + 1).to_string(),
            repo.name.clone(),
            repo.stargazers_count.to_string(),
            repo.fork_count.to_string(),
            repo.language.clone().unwrap_or_default(),
            repo.html_url.clone(),
            "".to_string(),
            repo.open_issues_count.to_string(),
            repo.pushed_at.clone(),
            repo.description.clone().unwrap_or_default(),
        ])?;
    }
    wtr.flush()?;
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
                // If no display name provided, use the API name.
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
    mappings
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments.
    let args = Args::parse();
    // Ensure output directory exists.
    fs::create_dir_all(&args.output)?;

    // Read GitHub token.
    let token = get_access_token()?;
    let client = Client::builder().build()?;
    // Parse languages.
    let languages = parse_languages(args.languages);

    // For each language, fetch repositories and write CSV.
    for mapping in languages {
        println!("Processing language: {} ({})", mapping.display_name, mapping.api_name);
        let repos = fetch_top_repos_for_language(&client, &token, &mapping.api_name, args.records).await?;
        // Build a file path in the output folder. File name: display name with non-alphanumeric characters replaced.
        let safe_name: String = mapping
            .display_name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();
        let file_path = format!("{}/{}.csv", args.output, safe_name);
        write_repos_to_csv(&file_path, &repos)?;
        println!("Saved {} records for {} in {}", repos.len(), mapping.display_name, file_path);
    }
    Ok(())
}
