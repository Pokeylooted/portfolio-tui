use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::processor::formatter::FormattedPortfolio;

/// Render the about view
#[allow(dead_code)]
pub fn render(frame: &mut Frame, area: Rect, portfolio: &FormattedPortfolio) {
    // Get terminal size
    let terminal_width = area.width as usize;
    
    // Create main layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Title
            Constraint::Length(1),  // Separator
            Constraint::Min(0),     // Content
            Constraint::Length(2),  // Navigation help
        ])
        .split(area);

    // Render title
    let title = Paragraph::new("About Me")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(title, main_chunks[0]);
    
    // Render separator
    let separator = "─".repeat(terminal_width);
    let separator_widget = Paragraph::new(separator)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(separator_widget, main_chunks[1]);

    // Create content layout
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),     // About text
            Constraint::Length(6),  // Social links
        ])
        .split(main_chunks[2]);

    // Render about text
    let about = Paragraph::new(portfolio.about.clone())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });
    frame.render_widget(about, content_chunks[0]);

    // Render social links
    let mut social_text = Vec::new();
    social_text.push(Line::from(vec![
        Span::styled("Social Links:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]));
    
    // Create a row of social icons
    let mut icon_spans = Vec::new();
    for social in &portfolio.social {
        let icon_text = match social.platform.to_lowercase().as_str() {
            "github" => "[GitHub]".to_string(),
            "twitter" => "[Twitter]".to_string(),
            "linkedin" => "[LinkedIn]".to_string(),
            "website" => "[Web]".to_string(),
            _ => format!("[{}]", social.platform),
        };
        
        icon_spans.push(Span::styled(icon_text, Style::default().fg(Color::White).bg(Color::DarkGray)));
        icon_spans.push(Span::raw(" "));
    }
    social_text.push(Line::from(icon_spans));
    
    // Add each social link
    for social in &portfolio.social {
        social_text.push(Line::from(vec![
            Span::styled(format!("{}: ", social.platform), Style::default().fg(Color::Blue)),
            Span::raw(&social.url),
        ]));
    }
    
    let social = Paragraph::new(social_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(social, content_chunks[1]);
    
    // Render navigation help
    let nav_text = vec![
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::raw(" for Home, "),
            Span::styled("p", Style::default().fg(Color::Yellow)),
            Span::raw(" for Projects, "),
            Span::styled("s", Style::default().fg(Color::Yellow)),
            Span::raw(" for Skills, "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" to quit"),
        ]),
    ];
    let nav = Paragraph::new(nav_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(nav, main_chunks[3]);
}