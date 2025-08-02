use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use crate::config::Args;
use crate::data::{fetcher::Fetcher, parser::Parser, Portfolio};
use crate::processor::Formatter;
use crate::ui::views::{self, View};

/// Application state
pub struct App {
    /// Terminal
    terminal: Terminal<CrosstermBackend<Stdout>>,
    /// Portfolio data
    portfolio: Option<Portfolio>,
    /// Formatted portfolio data
    formatted_portfolio: Option<crate::processor::formatter::FormattedPortfolio>,
    /// Current view
    current_view: View,
    /// Should quit
    should_quit: bool,
    /// Config path
    config_path: String,
    /// Available content sections
    content_sections: Vec<String>,
    /// Current section index
    current_section_index: usize,
}

impl App {
    /// Create a new application
    pub fn new(args: Args) -> Result<Self> {
        // Setup terminal
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            portfolio: None,
            formatted_portfolio: None,
            current_view: View::Home,
            should_quit: false,
            config_path: args.config_path,
            content_sections: Vec::new(),
            current_section_index: 0,
        })
    }

    /// Run the application
    pub async fn run(&mut self) -> Result<()> {
        // Load data
        self.load_data().await?;

        // Main loop
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        while !self.should_quit {
            // Handle events
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key.code);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.tick();
                last_tick = Instant::now();
            }

            // Render
            self.render()?;
        }

        // Restore terminal
        disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?;

        Ok(())
    }

    /// Load data from source
    async fn load_data(&mut self) -> Result<()> {
        // Create fetcher, parser, and formatter
        let fetcher = Fetcher::new();
        let parser = Parser::new();
        let formatter = Formatter::new();
        
        // Fetch data from source
        let content = fetcher.fetch(&self.config_path).await?;
        
        // Parse data
        let portfolio = parser.parse(&content)?;
        
        // Store the portfolio data
        self.portfolio = Some(portfolio.clone());
        
        // Format the portfolio data for display
        self.formatted_portfolio = Some(formatter.format(&portfolio));
        
        // Extract content sections for navigation
        self.extract_content_sections();
        
        Ok(())
    }
    
    /// Extract content sections for navigation
    fn extract_content_sections(&mut self) {
        if let Some(ref formatted_portfolio) = self.formatted_portfolio {
            // Always include Home as the first section
            self.content_sections = vec!["Home".to_string()];
            
            // Add all content sections from the portfolio
            for section in &formatted_portfolio.content_sections {
                self.content_sections.push(section.title.clone());
            }
        }
    }

    /// Handle key events
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => {
                self.current_section_index = 0; // Home is always the first section
                self.current_view = View::Home;
            },
            KeyCode::Left | KeyCode::Char('j') => self.previous_section(),
            KeyCode::Right | KeyCode::Char('l') => self.next_section(),
            KeyCode::Char(c) => {
                // Handle numeric keys for direct section navigation
                if let Some(digit) = c.to_digit(10) {
                    let index = if digit == 0 { 9 } else { (digit - 1) as usize };
                    if index < self.content_sections.len() {
                        self.navigate_to_section(index);
                    }
                } else {
                    // Handle first letter navigation (except for reserved keys)
                    if c != 'q' && c != 'h' && c != 'j' && c != 'l' {
                        for (i, section) in self.content_sections.iter().enumerate() {
                            if !section.is_empty() && section.to_lowercase().starts_with(c.to_lowercase().next().unwrap()) {
                                self.navigate_to_section(i);
                                break;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
    
    /// Navigate to the previous section
    fn previous_section(&mut self) {
        if !self.content_sections.is_empty() {
            self.current_section_index = if self.current_section_index == 0 {
                self.content_sections.len() - 1
            } else {
                self.current_section_index - 1
            };
            self.update_view_from_section();
        }
    }
    
    /// Navigate to the next section
    fn next_section(&mut self) {
        if !self.content_sections.is_empty() {
            self.current_section_index = (self.current_section_index + 1) % self.content_sections.len();
            self.update_view_from_section();
        }
    }
    
    /// Navigate to a specific section by index
    fn navigate_to_section(&mut self, index: usize) {
        if index < self.content_sections.len() {
            self.current_section_index = index;
            self.update_view_from_section();
        }
    }
    
    /// Update the current view based on the selected section
    fn update_view_from_section(&mut self) {
        if self.content_sections.is_empty() {
            return;
        }
        
        let section_name = &self.content_sections[self.current_section_index];
        
        // Set the view based on the section name
        self.current_view = match section_name.as_str() {
            "Home" => View::Home,
            _ => {
                // Find the index in the content_sections that corresponds to the formatted_portfolio.content_sections
                // Home is at index 0, so content sections start at index 1
                let content_index = if self.current_section_index > 0 {
                    self.current_section_index - 1
                } else {
                    0 // Fallback to first content section if something goes wrong
                };
                
                View::Content(content_index)
            }
        };
    }

    /// Update state
    fn tick(&mut self) {
        // TODO: Implement state updates
    }

    /// Render the UI
    fn render(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let size = frame.size();
            
            // Create a block for the entire UI
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Portfolio Viewer ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            
            // Render the block
            let inner_area = block.inner(size);
            frame.render_widget(block, size);
            
            // Render the current view
            if let Some(ref formatted_portfolio) = self.formatted_portfolio {
                match self.current_view {
                    View::Home => views::home::render(frame, inner_area, formatted_portfolio, &self.content_sections),
                    View::Content(index) => {
                        if index < formatted_portfolio.content_sections.len() {
                            views::content::render(frame, inner_area, formatted_portfolio, index);
                        } else {
                            // Fallback to home view if the index is out of bounds
                            views::home::render(frame, inner_area, formatted_portfolio, &self.content_sections);
                        }
                    }
                }
            } else {
                // Render loading message if portfolio data is not loaded yet
                let loading = Paragraph::new("Loading portfolio data...")
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::NONE));
                frame.render_widget(loading, inner_area);
            }
        })?;

        Ok(())
    }
}