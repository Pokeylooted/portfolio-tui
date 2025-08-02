use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::processor::formatter::FormattedPortfolio;

/// Render the projects view
pub fn render(frame: &mut Frame, area: Rect, portfolio: &FormattedPortfolio) {
    // Get terminal size
    let terminal_width = area.width as usize;
    
    // Create main layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Title
            Constraint::Length(1),  // Separator
            Constraint::Min(0),     // Projects content
            Constraint::Length(2),  // Navigation help
        ])
        .split(area);

    // Render title
    let title = Paragraph::new("Projects")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(title, main_chunks[0]);
    
    // Render separator
    let separator = "─".repeat(terminal_width);
    let separator_widget = Paragraph::new(separator)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(separator_widget, main_chunks[1]);

    // Render projects list
    let mut projects_content = Vec::new();
    
    for project in &portfolio.projects {
        // Project title with GitHub icon
        projects_content.push(Line::from(vec![
            Span::styled(&project.name, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
        
        // GitHub URL
        if !project.url.is_empty() {
            projects_content.push(Line::from(vec![
                Span::raw(&project.url),
            ]));
        }
        
        // GitHub icon (simulated with text)
        projects_content.push(Line::from(vec![
            Span::raw("[GitHub]"),
        ]));
        
        // Description
        projects_content.push(Line::from(vec![
            Span::raw(&project.description),
        ]));
        
        // Technologies
        if !project.technologies.is_empty() {
            let tech_spans: Vec<Span> = project.technologies.iter()
                .map(|tech| {
                    Span::styled(format!("{} ", tech), Style::default().fg(Color::Green))
                })
                .collect();
            
            projects_content.push(Line::from(tech_spans));
        }
        
        // Add empty line between projects
        projects_content.push(Line::from(vec![Span::raw("")]));
    }

    let projects_widget = Paragraph::new(projects_content)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White));
    frame.render_widget(projects_widget, main_chunks[2]);
    
    // Render navigation help
    let nav_text = vec![
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::raw(" for Home, "),
            Span::styled("s", Style::default().fg(Color::Yellow)),
            Span::raw(" for Skills, "),
            Span::styled("a", Style::default().fg(Color::Yellow)),
            Span::raw(" for About, "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" to quit"),
        ]),
    ];
    let nav = Paragraph::new(nav_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(nav, main_chunks[3]);
}