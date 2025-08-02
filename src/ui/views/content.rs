use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::processor::formatter::FormattedPortfolio;

/// Render a dynamic content section
pub fn render(frame: &mut Frame, area: Rect, portfolio: &FormattedPortfolio, section_index: usize) {
    // Get terminal size
    let terminal_width = area.width as usize;
    
    // Create main layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(1),  // Separator
            Constraint::Min(10),    // Content
            Constraint::Length(2),  // Navigation help
        ])
        .margin(1)  // Add margin to prevent text from touching the borders
        .split(area);
    
    // Get the content section
    if section_index >= portfolio.content_sections.len() {
        // Render error message if section index is out of bounds
        let error_message = Paragraph::new("Error: Content section not found")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(error_message, main_chunks[2]);
        return;
    }
    
    let section = &portfolio.content_sections[section_index];
    
    // Render section title
    let title = Paragraph::new(section.title.clone())
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(title, main_chunks[0]);
    
    // Render separator
    let separator = "─".repeat(terminal_width);
    let separator_widget = Paragraph::new(separator)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(separator_widget, main_chunks[1]);
    
    // Render content based on layout
    match section.layout.as_str() {
        "list" => render_list_content(frame, main_chunks[2], section),
        "text" => render_text_content(frame, main_chunks[2], section),
        _ => render_default_content(frame, main_chunks[2], section),
    }
    
    // Render navigation help
    let nav_text = vec![
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::raw(" for Home, "),
            Span::styled("←/→", Style::default().fg(Color::Yellow)),
            Span::raw(" to navigate sections, "),
            Span::styled("0-9", Style::default().fg(Color::Yellow)),
            Span::raw(" for direct section access, "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" to quit"),
        ]),
    ];
    let nav = Paragraph::new(nav_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(nav, main_chunks[3]);
}

/// Render content in list layout
fn render_list_content(frame: &mut Frame, area: Rect, section: &crate::processor::formatter::FormattedContentSection) {
    if section.items.is_empty() {
        let empty_message = Paragraph::new("No items to display")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(empty_message, area);
        return;
    }
    
    // Calculate how many items we can display
    let item_height = 6; // Approximate height for each item
    let visible_items = (area.height as usize / item_height).max(1);
    
    // Create constraints for items
    let mut constraints = Vec::with_capacity(visible_items);
    for _ in 0..visible_items.min(section.items.len()) {
        constraints.push(Constraint::Length(item_height as u16));
    }
    if constraints.is_empty() {
        constraints.push(Constraint::Min(1));
    }
    
    // Create layout for items
    let item_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    
    // Render each visible item
    for (i, item) in section.items.iter().enumerate().take(visible_items) {
        if i >= item_chunks.len() {
            break;
        }
        
        // Create item layout
        let item_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Title
                Constraint::Length(1), // Subtitle/Caption
                Constraint::Min(2),    // Description/Quote
            ])
            .split(item_chunks[i]);
        
        // Render title
        if !item.title.is_empty() {
            let title = Paragraph::new(item.title.clone())
                .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(title, item_layout[0]);
        }
        
        // Render subtitle or caption
        let subtitle_text = if !item.sub_title.is_empty() {
            item.sub_title.clone()
        } else if !item.caption.is_empty() {
            item.caption.clone()
        } else {
            String::new()
        };
        
        if !subtitle_text.is_empty() {
            let subtitle = Paragraph::new(subtitle_text)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(subtitle, item_layout[1]);
        }
        
        // Render description or quote
        let content_text = if !item.description.is_empty() {
            item.description.clone()
        } else if !item.quote.is_empty() {
            format!("> {}", item.quote)
        } else {
            String::new()
        };
        
        if !content_text.is_empty() {
            // Create a paragraph with proper wrapping
            let content = Paragraph::new(content_text)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::NONE))
                .wrap(Wrap { trim: true });
            
            frame.render_widget(content, item_layout[2]);
        }
    }
}

/// Render content in text layout
fn render_text_content(frame: &mut Frame, area: Rect, section: &crate::processor::formatter::FormattedContentSection) {
    if section.items.is_empty() {
        let empty_message = Paragraph::new("No content to display")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(empty_message, area);
        return;
    }
    
    // For text layout, we just display the description of the first item
    let content_text = if !section.items[0].description.is_empty() {
        section.items[0].description.clone()
    } else {
        "No text content available".to_string()
    };
    
    let content = Paragraph::new(content_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });
    
    frame.render_widget(content, area);
}

/// Render content with default layout
fn render_default_content(frame: &mut Frame, area: Rect, section: &crate::processor::formatter::FormattedContentSection) {
    // Default to list layout
    render_list_content(frame, area, section);
}