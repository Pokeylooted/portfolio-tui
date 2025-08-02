use serde::{Deserialize, Serialize};

/// Portfolio data model
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Portfolio {
    /// Repository
    pub repository: Option<String>,
    /// Favicon
    pub favicon: Option<String>,
    /// Version
    pub version: Option<String>,
    /// Name
    pub name: Option<String>,
    /// Title
    pub title: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Website
    pub website: Option<String>,
    /// Dark mode
    pub darkmode: Option<bool>,
    /// Twitter username
    pub twitter_username: Option<String>,
    /// GitHub username
    pub github_username: Option<String>,
    /// Discord username
    pub discord_username: Option<String>,
    /// StackOverflow username
    pub stackoverflow_username: Option<String>,
    /// LinkedIn username
    pub linkedin_username: Option<String>,
    /// Additional links
    pub additional_links: Option<Vec<AdditionalLink>>,
    /// About profile image
    pub about_profile_image: Option<String>,
    /// About content
    pub about_content: Option<String>,
    /// Content sections
    pub content: Option<Vec<ContentSection>>,
}

/// Additional link
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdditionalLink {
    /// Title
    pub title: Option<String>,
    /// Icon
    pub icon: Option<String>,
    /// URL
    pub url: Option<String>,
}

/// Content section
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentSection {
    /// Title
    pub title: Option<String>,
    /// Layout
    pub layout: Option<String>,
    /// Content items - can be either a list of items or a string
    #[serde(default)]
    pub content: ContentValue,
}

/// Content value - can be either a list of items or a string
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ContentValue {
    /// List of content items
    Items(Vec<ContentItem>),
    /// String content
    Text(String),
    /// Empty content
    #[serde(skip_deserializing)]
    Empty,
}

impl Default for ContentValue {
    fn default() -> Self {
        ContentValue::Empty
    }
}

/// Content item
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentItem {
    /// Layout
    pub layout: Option<String>,
    /// Title
    pub title: Option<String>,
    /// Sub-title
    pub sub_title: Option<String>,
    /// Caption
    pub caption: Option<String>,
    /// Icon
    pub icon: Option<String>,
    /// URL
    pub url: Option<String>,
    /// Quote
    pub quote: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Link
    pub link: Option<String>,
    /// Additional links
    pub additional_links: Option<Vec<AdditionalLink>>,
}

// For compatibility with the existing code
/// Personal information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Personal {
    /// Name
    pub name: String,
    /// Title
    pub title: String,
    /// Email
    pub email: Option<String>,
    /// About
    pub about: Option<String>,
}

/// Project information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Project URL
    pub url: Option<String>,
    /// Technologies used
    pub technologies: Option<Vec<String>>,
}

/// Skill information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Skill {
    /// Skill name
    pub name: String,
    /// Skill level (1-5)
    pub level: Option<u8>,
    /// Skill category
    pub category: Option<String>,
}

/// Social link
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Social {
    /// Platform name
    pub platform: String,
    /// URL
    pub url: String,
    /// Username
    pub username: Option<String>,
}