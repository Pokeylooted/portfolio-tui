use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::processor::formatter::FormattedPortfolio;
use crate::ui::ascii_art;

/// Render the home view
pub fn render(frame: &mut Frame, area: Rect, portfolio: &FormattedPortfolio, content_sections: &[String]) {
    // Get terminal size
    let terminal_width = area.width as usize;
    
    // Create main layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Logo and header
            Constraint::Length(1),  // Separator
            Constraint::Min(10),    // Content
            Constraint::Length(2),  // Navigation help
        ])
        .margin(1)  // Add margin to improve spacing
        .split(area);
    
    // Create header layout (logo + name/title)
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Logo
            Constraint::Percentage(80), // Name and title
        ])
        .split(main_chunks[0]);
    
    // Render ASCII logo
    let logo = Paragraph::new(ascii_art::get_logo("duck"))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(logo, header_chunks[0]);
    
    // Render title and name
    let name_title = vec![
        Line::from(vec![
            Span::raw(&portfolio.title),
            Span::raw(" "),
            Span::styled(&portfolio.name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::styled("Email: ", Style::default().fg(Color::Blue)),
            Span::raw(portfolio.social.iter().find(|s| s.platform == "Email").map_or_else(|| "N/A".to_string(), |s| s.username.clone())),
        ]),
        Line::from(vec![
            Span::styled("Web: ", Style::default().fg(Color::Blue)),
            Span::raw(portfolio.social.iter().find(|s| s.platform == "Website").map_or_else(|| "N/A".to_string(), |s| s.url.clone())),
        ]),
    ];
    
    let name_title_widget = Paragraph::new(name_title)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(name_title_widget, header_chunks[1]);
    
    // Render separator
    let separator = "─".repeat(terminal_width);
    let separator_widget = Paragraph::new(separator)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(separator_widget, main_chunks[1]);
    
    // Render about section
    let about_title = Paragraph::new("About Me")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    
    let about_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // About title
            Constraint::Min(0),     // About content
        ])
        .split(main_chunks[2]);
    
    frame.render_widget(about_title, about_chunks[0]);
    
    // Calculate available width for text
    let available_width = about_chunks[1].width as usize - 4; // Subtract some padding
    
    // Create a formatter with the appropriate width
    let formatter = crate::processor::Formatter::with_max_width(available_width);
    let processed_about = formatter.process_text(&portfolio.about);
    
    let about_content = Paragraph::new(processed_about)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });
    frame.render_widget(about_content, about_chunks[1]);
    
    // Render navigation help
    let mut nav_spans = vec![
        Span::raw("Press "),
        Span::styled("←/→", Style::default().fg(Color::Yellow)),
        Span::raw(" to navigate, "),
    ];
    
    // Add section shortcuts
    for (i, section) in content_sections.iter().enumerate().take(10) {
        if i > 0 {
            nav_spans.push(Span::raw(", "));
        }
        
        let key = if i == 0 {
            "h".to_string()
        } else if i < 10 {
            i.to_string()
        } else {
            continue;
        };
        
        nav_spans.push(Span::styled(key, Style::default().fg(Color::Yellow)));
        nav_spans.push(Span::raw(format!(" for {}", section)));
    }
    
    nav_spans.push(Span::raw(", "));
    nav_spans.push(Span::styled("q", Style::default().fg(Color::Yellow)));
    nav_spans.push(Span::raw(" to quit"));
    
    let nav_text = vec![Line::from(nav_spans)];
    let nav = Paragraph::new(nav_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(nav, main_chunks[3]);
}