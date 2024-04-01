mod app;
mod constants;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::disable_raw_mode,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

use crate::ui::*;

use crate::app::*;

use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Url);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('c') => {
                        app.current_screen = CurrentScreen::Clearing;
                    }

                    KeyCode::Up => App::on_arrow_up(app),
                    KeyCode::Down => App::on_arrow_down(app),
                    _ => {}
                },
                CurrentScreen::Clearing => match key.code {
                    KeyCode::Char('y') => {
                        App::on_press_c(app);
                        app.current_screen = CurrentScreen::Main
                    }
                    KeyCode::Char('n') => app.current_screen = CurrentScreen::Main,
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => return Ok(true),
                    KeyCode::Char('n') | KeyCode::Char('q') => return Ok(false),
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => App::on_press_enter(app),
                    KeyCode::Backspace => App::on_press_backspace(app),
                    KeyCode::Esc => App::on_press_esc(app),
                    KeyCode::Tab => App::on_press_tab(app),
                    KeyCode::Char(value) => App::on_char_press(app, value),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
