use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::processor::formatter::FormattedPortfolio;

/// Render the skills view
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
            Constraint::Min(0),     // Skills content
            Constraint::Length(2),  // Navigation help
        ])
        .split(area);

    // Render title
    let title = Paragraph::new("Skills")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(title, main_chunks[0]);
    
    // Render separator
    let separator = "─".repeat(terminal_width);
    let separator_widget = Paragraph::new(separator)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(separator_widget, main_chunks[1]);

    // Group skills by category
    let mut categories: std::collections::HashMap<String, Vec<&crate::processor::formatter::FormattedSkill>> =
        std::collections::HashMap::new();
    
    for skill in &portfolio.skills {
        categories
            .entry(skill.category.clone())
            .or_insert_with(Vec::new)
            .push(skill);
    }

    // Render skills list
    let mut skills_content = Vec::new();
    
    for (category, skills) in categories {
        // Category title
        let category_name = category.clone();
        skills_content.push(Line::from(vec![
            Span::styled(category_name.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
        
        // Add a line after the category title
        skills_content.push(Line::from(vec![
            Span::styled("─".repeat(category_name.len()), Style::default().fg(Color::DarkGray)),
        ]));
        
        // Create a list of skills with badges
        let mut skill_spans = Vec::new();
        for skill in skills {
            // Create a badge-like display for each skill
            let badge = format!(" {} ", skill.name);
            skill_spans.push(
                Span::styled(badge, Style::default().fg(Color::Black).bg(Color::Green))
            );
            skill_spans.push(Span::raw(" "));
        }
        
        skills_content.push(Line::from(skill_spans));
        
        // Add empty line between categories
        skills_content.push(Line::from(vec![Span::raw("")]));
    }

    // Add a section for "I am most skilled in:" similar to the website
    skills_content.push(Line::from(vec![
        Span::raw("I am most skilled in: "),
        Span::styled("FastAPI", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" and "),
        Span::styled("Eating Pizza", Style::default().fg(Color::Black).bg(Color::White)),
    ]));

    let skills_widget = Paragraph::new(skills_content)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White));
    frame.render_widget(skills_widget, main_chunks[2]);
    
    // Render navigation help
    let nav_text = vec![
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::raw(" for Home, "),
            Span::styled("p", Style::default().fg(Color::Yellow)),
            Span::raw(" for Projects, "),
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