use anyhow::Result;
use serde_yaml;

use super::models::Portfolio;

/// Parses YAML content into structured data
pub struct Parser;

impl Parser {
    /// Create a new parser
    pub fn new() -> Self {
        Self
    }

    /// Parse YAML content into a Portfolio
    pub fn parse(&self, content: &str) -> Result<Portfolio> {
        let portfolio: Portfolio = serde_yaml::from_str(content)?;
        Ok(portfolio)
    }
}