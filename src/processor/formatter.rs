use crate::data::models::{Portfolio, ContentValue, ContentSection};
use regex::Regex;
use textwrap::wrap;

/// Formats data for display
pub struct Formatter {
    /// Maximum width for text wrapping
    max_width: usize,
}

impl Formatter {
    /// Create a new formatter
    pub fn new() -> Self {
        Self {
            max_width: 100, // Default width to ensure text fits within UI boundaries
        }
    }

    /// Create a new formatter with a specific max width
    pub fn with_max_width(max_width: usize) -> Self {
        Self {
            max_width,
        }
    }

    /// Process text to remove Markdown syntax and format for display
    pub fn process_text(&self, text: &str) -> String {
        let text = self.remove_mark_tags(text);
        let text = self.remove_image_references(&text);
        self.wrap_text(&text)
    }

    /// Remove <mark> tags but keep the text inside
    fn remove_mark_tags(&self, text: &str) -> String {
        // Replace <mark> and </mark> with nothing
        text.replace("<mark>", "").replace("</mark>", "")
    }

    /// Remove image references like ![Developing](/images/tony.jpg "Developing")
    fn remove_image_references(&self, text: &str) -> String {
        // Match pattern: ![alt text](/path/to/image.jpg "title")
        let re = Regex::new(r#"!\[.*?\]\(.*?(?:\s+".*?")?\)"#).unwrap();
        re.replace_all(text, "").to_string()
    }

    /// Wrap text to fit within UI boundaries
    fn wrap_text(&self, text: &str) -> String {
        // Split by newlines and wrap each paragraph
        let paragraphs: Vec<String> = text
            .split('\n')
            .map(|para| {
                if para.trim().is_empty() {
                    String::from("")
                } else {
                    wrap(para, self.max_width).join("\n")
                }
            })
            .collect();

        // Join paragraphs back with newlines
        paragraphs.join("\n")
    }

    /// Format portfolio data for display
    pub fn format(&self, portfolio: &Portfolio) -> FormattedPortfolio {
        // Extract personal information
        let name = portfolio.name.clone().unwrap_or_else(|| "Pokeylooted".to_string());
        let title = portfolio.title.clone().unwrap_or_else(|| "Developer".to_string());
        let about = portfolio.about_content.clone().unwrap_or_default();
        let processed_about = self.process_text(&about);
        
        // Extract all content sections
        let mut content_sections = Vec::new();
        if let Some(sections) = &portfolio.content {
            for section in sections {
                content_sections.push(self.format_content_section(section));
            }
        }
        
        // Extract projects (for backward compatibility)
        let mut projects = Vec::new();
        if let Some(content_sections) = &portfolio.content {
            for section in content_sections {
                if section.title.as_deref() == Some("Projects") {
                    match &section.content {
                        ContentValue::Items(items) => {
                            for item in items {
                                if let (Some(title), Some(description)) = (&item.title, &item.description) {
                                    let processed_description = self.process_text(description);
                                    projects.push(FormattedProject {
                                        name: title.clone(),
                                        description: processed_description,
                                        url: item.url.clone().unwrap_or_default(),
                                        technologies: Vec::new(), // No technologies in the YAML
                                    });
                                }
                            }
                        },
                        ContentValue::Text(text) => {
                            // Handle text content if needed
                            let _processed_text = self.process_text(text);
                            // Could create a project from text if needed
                        },
                        ContentValue::Empty => {
                            // Handle empty content
                        }
                    }
                }
            }
        }
        
        // Extract skills from about section
        let mut skills = Vec::new();
        if let Some(about) = &portfolio.about_content {
            // Look for skills in the about section (marked with <mark> tags)
            let re = Regex::new(r#"<mark>(.*?)</mark>"#).unwrap();
            for cap in re.captures_iter(about) {
                if let Some(skill_match) = cap.get(1) {
                    let skill_name = skill_match.as_str().to_string();
                    skills.push(FormattedSkill {
                        name: skill_name,
                        _level: 5, // Default level
                        category: "Skill".to_string(), // Default category
                    });
                }
            }
        }
        
        // If no skills were found, create some sample skills
        if skills.is_empty() {
            skills = vec![
                FormattedSkill {
                    name: "FastAPI".to_string(),
                    _level: 5,
                    category: "Backend".to_string(),
                },
                FormattedSkill {
                    name: "Eating Pizza".to_string(),
                    _level: 5,
                    category: "Food".to_string(),
                },
            ];
        }
        
        // Extract social links
        let mut social = Vec::new();
        
        // Add Email
        if let Some(email) = &portfolio.email {
            social.push(FormattedSocial {
                platform: "Email".to_string(),
                url: format!("mailto:{}", email),
                username: email.clone(),
            });
        }
        
        // Add Website
        if let Some(website) = &portfolio.website {
            social.push(FormattedSocial {
                platform: "Website".to_string(),
                url: website.clone(),
                username: website.clone(),
            });
        }
        
        // Add GitHub
        if let Some(username) = &portfolio.github_username {
            social.push(FormattedSocial {
                platform: "GitHub".to_string(),
                url: format!("https://github.com/{}", username),
                username: username.clone(),
            });
        }
        
        // Add Twitter
        if let Some(username) = &portfolio.twitter_username {
            social.push(FormattedSocial {
                platform: "Twitter".to_string(),
                url: format!("https://twitter.com/{}", username),
                username: username.clone(),
            });
        }
        
        // Add LinkedIn
        if let Some(username) = &portfolio.linkedin_username {
            social.push(FormattedSocial {
                platform: "LinkedIn".to_string(),
                url: format!("https://linkedin.com/in/{}", username),
                username: username.clone(),
            });
        }
        
        // Add Discord
        if let Some(username) = &portfolio.discord_username {
            social.push(FormattedSocial {
                platform: "Discord".to_string(),
                url: format!("https://discord.com/users/{}", username),
                username: username.clone(),
            });
        }
        
        FormattedPortfolio {
            name,
            title,
            about: processed_about,
            content_sections,
            projects,
            skills,
            social,
        }
    }

    /// Format a content section
    fn format_content_section(&self, section: &ContentSection) -> FormattedContentSection {
        let title = section.title.clone().unwrap_or_default();
        let layout = section.layout.clone().unwrap_or_default();
        
        let items = match &section.content {
            ContentValue::Items(content_items) => {
                content_items.iter().map(|item| {
                    let title = item.title.clone().unwrap_or_default();
                    let sub_title = item.sub_title.clone().unwrap_or_default();
                    let caption = item.caption.clone().unwrap_or_default();
                    let description = item.description.clone().unwrap_or_default();
                    let processed_description = self.process_text(&description);
                    let quote = item.quote.clone().unwrap_or_default();
                    let processed_quote = self.process_text(&quote);
                    let _url = item.url.clone().unwrap_or_default();
                    let _link = item.link.clone().unwrap_or_default();
                    
                    FormattedContentItem {
                        title,
                        sub_title,
                        caption,
                        description: processed_description,
                        quote: processed_quote,
                        _url,
                        _link,
                    }
                }).collect()
            },
            ContentValue::Text(text) => {
                let processed_text = self.process_text(text);
                vec![FormattedContentItem {
                    title: String::new(),
                    sub_title: String::new(),
                    caption: String::new(),
                    description: processed_text,
                    quote: String::new(),
                    _url: String::new(),
                    _link: String::new(),
                }]
            },
            ContentValue::Empty => Vec::new(),
        };
        
        FormattedContentSection {
            title,
            layout,
            items,
        }
    }
}

/// Formatted portfolio data for display
#[derive(Debug, Clone)]
pub struct FormattedPortfolio {
    pub name: String,
    pub title: String,
    pub about: String,
    pub content_sections: Vec<FormattedContentSection>,
    pub projects: Vec<FormattedProject>,
    pub skills: Vec<FormattedSkill>,
    pub social: Vec<FormattedSocial>,
}

/// Formatted content section
#[derive(Debug, Clone)]
pub struct FormattedContentSection {
    pub title: String,
    pub layout: String,
    pub items: Vec<FormattedContentItem>,
}

/// Formatted content item
#[derive(Debug, Clone)]
pub struct FormattedContentItem {
    pub title: String,
    pub sub_title: String,
    pub caption: String,
    pub description: String,
    pub quote: String,
    // These fields are kept for future use but marked with underscores to avoid warnings
    pub _url: String,
    pub _link: String,
}

/// Formatted project data for display
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormattedProject {
    pub name: String,
    pub description: String,
    pub url: String,
    pub technologies: Vec<String>,
}

/// Formatted skill data for display
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormattedSkill {
    pub name: String,
    pub _level: u8,  // Renamed to avoid warning
    pub category: String,
}

/// Formatted social link data for display
#[derive(Debug, Clone)]
pub struct FormattedSocial {
    pub platform: String,
    pub url: String,
    pub username: String,
}