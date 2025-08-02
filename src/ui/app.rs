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
        let content = fetcher.fetch("https://github.com/Pokeylooted/Pokeylooted.github.io/raw/main/_config.yml").await?;
        
        // Parse data
        let portfolio = parser.parse(&content)?;
        
        // Store the portfolio data
        self.portfolio = Some(portfolio.clone());
        
        // Format the portfolio data for display
        self.formatted_portfolio = Some(formatter.format(&portfolio));
        
        Ok(())
    }

    /// Handle key events
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.current_view = View::Home,
            KeyCode::Char('p') => self.current_view = View::Projects,
            KeyCode::Char('s') => self.current_view = View::Skills,
            KeyCode::Char('a') => self.current_view = View::About,
            _ => {}
        }
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
                    View::Home => views::home::render(frame, inner_area, formatted_portfolio),
                    View::Projects => views::projects::render(frame, inner_area, formatted_portfolio),
                    View::Skills => views::skills::render(frame, inner_area, formatted_portfolio),
                    View::About => views::about::render(frame, inner_area, formatted_portfolio),
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