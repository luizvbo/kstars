#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Repo {
    #[serde(rename = "Ranking")]
    pub ranking: u32,
    #[serde(rename = "Project Name")]
    pub project_name: String,
    #[serde(rename = "Stars")]
    pub stars: u64,
    #[serde(rename = "Forks")]
    pub forks: u64,
    #[serde(rename = "Watchers")]
    pub watchers: u64,
    #[serde(rename = "Open Issues")]
    pub open_issues: u64,
    #[serde(rename = "Created At")]
    pub created_at: String,
    #[serde(rename = "Last Commit")]
    pub last_commit: String,
    #[serde(rename = "Size")]
    pub size: String,
    #[serde(rename = "Size (KB)")]
    pub size_kb: u64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Repo URL")]
    pub repo_url: String,
}
