use serde::{Deserialize, Serialize};

/// Portfolio data model
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Portfolio {
    /// Personal information
    pub personal: Personal,
    /// Projects
    pub projects: Vec<Project>,
    /// Skills
    pub skills: Vec<Skill>,
    /// Social links
    pub social: Vec<Social>,
}

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