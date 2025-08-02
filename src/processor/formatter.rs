use crate::data::models::Portfolio;

/// Formats data for display
pub struct Formatter;

impl Formatter {
    /// Create a new formatter
    pub fn new() -> Self {
        Self
    }

    /// Format portfolio data for display
    pub fn format(&self, portfolio: &Portfolio) -> FormattedPortfolio {
        FormattedPortfolio {
            name: portfolio.personal.name.clone(),
            title: portfolio.personal.title.clone(),
            about: portfolio.personal.about.clone().unwrap_or_default(),
            projects: portfolio.projects.iter().map(|p| {
                FormattedProject {
                    name: p.name.clone(),
                    description: p.description.clone(),
                    url: p.url.clone().unwrap_or_default(),
                    technologies: p.technologies.clone().unwrap_or_default(),
                }
            }).collect(),
            skills: portfolio.skills.iter().map(|s| {
                FormattedSkill {
                    name: s.name.clone(),
                    level: s.level.unwrap_or(0),
                    category: s.category.clone().unwrap_or_default(),
                }
            }).collect(),
            social: portfolio.social.iter().map(|s| {
                FormattedSocial {
                    platform: s.platform.clone(),
                    url: s.url.clone(),
                    username: s.username.clone().unwrap_or_default(),
                }
            }).collect(),
        }
    }
}

/// Formatted portfolio data for display
#[derive(Debug, Clone)]
pub struct FormattedPortfolio {
    pub name: String,
    pub title: String,
    pub about: String,
    pub projects: Vec<FormattedProject>,
    pub skills: Vec<FormattedSkill>,
    pub social: Vec<FormattedSocial>,
}

/// Formatted project data for display
#[derive(Debug, Clone)]
pub struct FormattedProject {
    pub name: String,
    pub description: String,
    pub url: String,
    pub technologies: Vec<String>,
}

/// Formatted skill data for display
#[derive(Debug, Clone)]
pub struct FormattedSkill {
    pub name: String,
    pub level: u8,
    pub category: String,
}

/// Formatted social link data for display
#[derive(Debug, Clone)]
pub struct FormattedSocial {
    pub platform: String,
    pub url: String,
    pub username: String,
}