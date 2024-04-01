use crate::constants::TITLE;
use crate::{App, CurrentScreen, CurrentlyEditing};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear, List, ListItem, Paragraph, Wrap,
    },
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let active_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::Green);

    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(63),
                Constraint::Percentage(7),
            ]
            .as_ref(),
        )
        .split(f.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(100),
            Constraint::Percentage(100),
        ])
        .split(main_layout_horizontal[1]);

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100)])
        .split(main_layout_horizontal[2]);

    let mut footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().add_modifier(Modifier::BOLD));

    let title_block = Block::default()
        .borders(Borders::ALL)
        .add_modifier(Modifier::BOLD)
        .title(
            create_current_navigation_text(app)
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .bold();

    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(title_block);

    let left_panel_block = Block::default()
        .borders(Borders::ALL)
        .add_modifier(Modifier::BOLD)
        .title(Title::from("| Request History |").alignment(Alignment::Center))
        .fg(match app.current_screen {
            CurrentScreen::Main => Color::Green,
            _ => Color::White,
        })
        .title(match app.current_screen {
            CurrentScreen::Main => {
                if app.requests.len() > 1 {
                    Title::from("| Use ↓↑ to move |")
                        .alignment(Alignment::Center)
                        .position(Position::Bottom)
                } else {
                    Title::from("")
                }
            }
            _ => Title::from(""),
        });

    let right_panel_block = Block::default()
        .borders(Borders::ALL)
        .add_modifier(Modifier::BOLD)
        .title(Title::from("| Content |").alignment(Alignment::Center));

    if let Some(editing) = &app.currently_editing {
        match editing {
            CurrentlyEditing::Url => footer_block = footer_block.style(active_style),
        };

        let value_text = Paragraph::new(app.url_input.clone())
            .block(footer_block.clone().add_modifier(Modifier::BOLD));
        f.render_widget(value_text, footer_layout[0]);
    }

    f.render_widget(title_paragraph, main_layout_horizontal[0]);
    f.render_widget(footer_block, footer_layout[0]);

    f.render_widget(
        create_history_list(app).block(left_panel_block),
        inner_layout[0],
    );
    f.render_widget(
        create_content_text(app).block(right_panel_block),
        inner_layout[1],
    );

    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area);
    }

    if let CurrentScreen::Clearing = app.current_screen {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("| Clear Request History |")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray))
            .add_modifier(Modifier::BOLD);

        let exit_text = Text::styled(
            "(Y) to confirm / (N) to cancel",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area);
    }
}

fn create_current_navigation_text(app: &App) -> Title<'_> {
    match app.current_screen {
        CurrentScreen::Main => Title::from("| (E) to edit / (Q) to quit / (C) to clear history |"),

        CurrentScreen::Editing => {
            Title::from("| Editing Mode - (ESC) to cancel / (ENTER) to complete |")
        }

        CurrentScreen::Exiting => Title::from("Exiting"),
        CurrentScreen::Clearing => {
            Title::from("| Clearing history - (Y) to confirm / (N) to cancel| ")
        }
    }
}

fn create_history_list(app: &App) -> List {
    let active_style = Style::default().bg(Color::Green).fg(Color::Black);

    let mut list_items = Vec::<ListItem>::new();

    for (index, key) in app.requests.keys().enumerate() {
        if app.current_index == index {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <100}", key),
                active_style,
            ))));
        } else {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <100}", key),
                Style::default(),
            ))));
        }
    }

    List::new(list_items)
}

fn create_content_text(app: &App) -> Paragraph {
    let mut content: String = String::from("");
    // let mut list_items = Vec::<ListItem>::new();

    for (index, key) in app.requests.keys().enumerate() {
        if index == app.current_index {
            content = app.requests.get(key).unwrap().to_string()
        }
    }

    Paragraph::new(Text::from(content.to_string())).wrap(Wrap { trim: true })
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
